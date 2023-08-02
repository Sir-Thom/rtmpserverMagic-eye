use actix_web::{web, HttpResponse, Responder};
use log::{error, info, warn};

use rtmp::channels::ChannelsManager;
use rtmp::rtmp::RtmpServer;

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize)]
pub struct RtmpServerManager {
    servers: Arc<Mutex<HashMap<u16, String>>>,
    server_id_counter: Mutex<u16>, // The server ID counter is now inside the RtmpServerManager
}

impl RtmpServerManager {
    pub fn new() -> Self {
        RtmpServerManager {
            servers: Arc::new(Mutex::new(HashMap::new())),
            server_id_counter: Mutex::new(0),
        }
    }

    // Function to create RTMP servers
    pub async fn create_rtmp_server(&self, num_servers: u16) -> anyhow::Result<()> {
        let mut channel = ChannelsManager::new(None);
        let producer = channel.get_channel_event_producer();

        let mut server_addresses = HashMap::new();

        for _ in 0..num_servers {
            let server_id = {
                let mut counter = self.server_id_counter.lock().unwrap();
                let id = *counter;
                *counter += 1;
                id
            };

            let base_ip = env::var("BASE_IP").unwrap_or_else(|_| "127.0.0.1".to_string());
            let base_port: u16 = env::var("BASE_PORT")
                .unwrap_or_else(|_| "1935".to_string())
                .parse()
                .unwrap();

            let ip = format!("{}", base_ip);
            let port = base_port + server_id;
            let address = format!("{ip}:{port}", ip = ip, port = port);

            let mut rtmp_server = RtmpServer::new(address.clone(), producer.clone());

            tokio::spawn(async move {
                if let Err(err) = rtmp_server.run().await {
                    error!("RTMP server error: {}", err);
                }
            });

            info!(
                "RTMP server {} started and listening on: {}",
                server_id, address
            );

            server_addresses.insert(server_id, address);
        }

        // Store the server addresses in the hashmap
        let mut servers = self.servers.lock().unwrap();
        servers.extend(server_addresses);
        tokio::spawn(async move { channel.run().await });

        Ok(())
    }

    pub fn get_all_rtmp_servers(&self) -> HashMap<u16, String> {
        self.servers.lock().unwrap().clone()
    }

    pub fn get_by_id_rtmp_servers(&self, id: u16) -> String {
        self.servers.lock().unwrap().get(&id).unwrap().clone()
    }
}

pub async fn create_rtmp_server_handler(
    server_manager: web::Data<RtmpServerManager>,
    num_servers: web::Path<u16>,
) -> impl Responder {
    let num_servers = num_servers.into_inner();
    if let Err(err) = server_manager.create_rtmp_server(num_servers).await {
        error!("Error creating RTMP servers: {}", err);
        HttpResponse::InternalServerError().body(format!("Error creating RTMP servers: {}", err))
    } else {
        info!("Successfully created {} RTMP servers", num_servers);
        HttpResponse::Ok().body(format!(
            "Successfully created {} RTMP servers!",
            num_servers
        ))
    }
}

// Actix-web handler

pub async fn get_all_rtmp_servers_handler(
    server_manager: web::Data<RtmpServerManager>,
) -> impl Responder {
    let servers: HashMap<u16, String> = server_manager.get_all_rtmp_servers();
    if servers.is_empty() {
        warn!("No RTMP servers running");
        return HttpResponse::NotFound().body("No RTMP servers running");
    }
    info!("Successfully retrieved all RTMP servers");
    json!(servers);
    HttpResponse::Ok().json(servers)
}

pub async fn get_by_id_rtmp_servers_handler(
    server_manager: web::Data<RtmpServerManager>,
    id: web::Path<u16>,
) -> impl Responder {
    let id = id.into_inner();
    let server = server_manager.get_by_id_rtmp_servers(id);
    if server != "" {
        info!("Successfully retrieved RTMP server");
        return HttpResponse::Ok().json(server);
    } else if server.is_empty() {
        warn!("No RTMP servers running");
        return HttpResponse::NotFound().body("No RTMP servers running");
    } else {
        error!("Error retrieving RTMP server");
        return HttpResponse::InternalServerError().body("Error retrieving RTMP server");
    }
}
