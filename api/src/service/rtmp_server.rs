use anyhow::Result;
use log::{error, info, warn};
use rtmp::rtmp::RtmpServer;
use std::collections::HashMap;
use std::env;
use streamhub::StreamsHub;
use tokio::sync::{Mutex, RwLock};

/// Struct to manage RTMP servers
///
/// # Attributes
/// * `servers` - The servers
/// * `server_id_counter` - The server ID counter
/// * `dynamic_ports` - The dynamic ports
pub struct RtmpServerManager {
    servers: Mutex<HashMap<u16, String>>,
    server_id_counter: Mutex<u16>,
    dynamic_ports: RwLock<Vec<u16>>,
    id_port_mapping: HashMap<u16, u16>,
}

impl RtmpServerManager {
    pub fn new() -> Self {
        RtmpServerManager {
            servers: Mutex::new(HashMap::new()),
            server_id_counter: Mutex::new(0),
            dynamic_ports: RwLock::new(Vec::new()),
            id_port_mapping: HashMap::new(),
        }
    }

    async fn get_next_dynamic_port(&self) -> u16 {
        let mut dynamic_ports = self.dynamic_ports.write().await;
        for port in 1935..=65535 {
            if !dynamic_ports.contains(&port) {
                dynamic_ports.push(port);
                return port;
            }
        }
        // Fallback to a default port if no available ports are found
        1935
    }

    pub async fn create_rtmp_server(&self, num_servers: u16) -> Result<Vec<(u16, String)>> {
        let mut stream_hub = StreamsHub::new(None);
        let sender = stream_hub.get_hub_event_sender();

        let mut server_addresses = Vec::new();

        for _ in 0..num_servers {
            let server_id = {
                let mut counter = self.server_id_counter.lock().await;
                let id = *counter;
                *counter += 1;
                id
            };

            let base_ip = env::var("BASE_IP").unwrap_or_else(|_| "0.0.0.0".to_string());
            let ip = format!("{}", base_ip);
            let port = self.get_next_dynamic_port().await; // Use the dynamically assigned port

            let mut _id_port_mapping = self.id_port_mapping.clone().insert(server_id, port);

            let address = format!("{ip}:{port}", ip = ip, port = port);
            //let stream1 = TcpStream::connect(address.clone()).await?;

            let mut rtmp_server = RtmpServer::new(address.clone(), sender.clone(), 1);

            tokio::spawn(async move {
                if let Err(err) = rtmp_server.run().await {
                    error!("RTMP server error: {}", err);
                }
            });

            info!(
                "RTMP server {} started and listening on: {}",
                server_id, address
            );

            server_addresses.push((server_id, address));
        }

        tokio::spawn(async move { stream_hub.run().await });

        Ok(server_addresses)
    }

    pub async fn get_all_rtmp_servers(&self) -> HashMap<u16, String> {
        if self.servers.lock().await.is_empty() {
            warn!("server list is empty please add some servers.")
        }
        self.servers.lock().await.clone()
    }

    pub async fn get_by_id_rtmp_servers(&self, id: u16) -> Option<String> {
        self.servers.lock().await.get(&id).cloned()
    }

    pub async fn remove_rtmp_server(&self, id: u16) {
        // Remove server from the servers HashMap
        let mut servers = self.servers.lock().await;
        servers.remove(&id);
        let mut stream_hub = StreamsHub::new(None);
        let sender = stream_hub.get_hub_event_sender();
        info!("stream hub sender {:?}", sender);
        // Remove server from stream_hub

        // Remove server from dynamic_ports vector and id_port_mapping HashMap
        if let Some(port) = self.id_port_mapping.get(&id) {
            let mut dynamic_ports = self.dynamic_ports.write().await;

            dynamic_ports.retain(|&p| p != *port);
            let mut _id_port_mapping = self.id_port_mapping.clone().remove(&id);
        }
    }
}
