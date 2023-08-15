pub use actix_web::{web, HttpResponse, Responder};
pub use log::{error, info, warn};
use rtmp::relay::pull_client::PullClient;
//depreacted
//use rtmp::channels::ChannelsManager;
pub use rtmp::rtmp::RtmpServer;
use lazy_static::lazy_static;
pub use serde_json::json;
use tokio::sync::RwLock;
pub use std::collections::HashMap;
use std::collections::HashSet;
pub use std::env;
pub use std::sync::{Arc, Mutex};
use streamhub::StreamsHub;
lazy_static! {
    static ref RTMP_SERVERS: RwLock<HashMap<u16, String>> = RwLock::new(HashMap::new());
}
/// Struct to manage RTMP servers
///
/// # Attributes
/// * `servers` - The servers
/// * `server_id_counter` - The server ID counter
///
/// # Example
/// ```
/// use api::service::rtmp_server::RtmpServerManager;
///     
/// let server_manager = RtmpServerManager::new();
/// ```
pub struct RtmpServerManager {
    servers: RTMP_SERVERS,
    served_ips: HashSet<String>,
    server_id_counter: Mutex<u16>, // The server ID counter is now inside the RtmpServerManager
}
/// Implementation of the RTMP server manager
///
/// # Example
/// ```
/// use api::service::rtmp_server::RtmpServerManager;
///
/// let server_manager = RtmpServerManager::new();
/// ```
impl RtmpServerManager {
    pub fn new() -> Self {
        RtmpServerManager {
            
            servers:RTMP_SERVERS,
            served_ips: HashSet<String>,
            server_id_counter: Mutex::new(0),
        }
    }

    /// Function to create RTMP servers
    ///
    /// # Arguments
    /// * `num_servers` - The number of servers to create
    ///
    /// # Returns
    /// * `anyhow::Result<()>` - The result
    ///
    /// # Example
    /// ```
    /// use api::service::rtmp_server::RtmpServerManager;
    ///
    /// let server_manager = RtmpServerManager::new();
    /// server_manager.create_rtmp_server(1);
    /// ```
    pub async fn create_rtmp_server(&self, num_servers: u16) -> anyhow::Result<()> {
        let mut stream_hub = StreamsHub::new(None);
        let sender = stream_hub.get_hub_event_sender();

        for _ in 0..num_servers {
           
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

            // Check if the IP address is already being served
            if self.served_ips.contains(&ip) {
                log::warn!("IP address {} is already being served", ip);
                continue; // Skip this server creation
            }

            // Update the set of served IPs
            self.served_ips.insert(ip.clone());

            //pull the rtmp stream from 192.168.0.3:1935 to local
            let address = format!("{ip}:{port}", ip = "192.168.0.3", port = "1935");
            log::info!("start rtmp pull client from address: {}", address);
            let mut pull_client = PullClient::new(
                address,
                stream_hub.get_client_event_consumer(),
                sender.clone(),
            );

            tokio::spawn(async move {
                if let Err(err) = pull_client.run().await {
                    log::error!("pull client error {}\n", err);
                }
            });
            stream_hub.set_rtmp_pull_enabled(true);
            //end pull
            let ip = format!("{}", base_ip);
            let port = base_port + server_id;
            let address = format!("127.0.0.1:{port}", port = port);

            let mut rtmp_server = RtmpServer::new(address.clone(), sender.clone(), 1);
            tokio::spawn(async move {
                if let Err(err) = rtmp_server.run().await {
                    log::error!("rtmp server error: {}\n", err);
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
        tokio::spawn(async move { stream_hub.run().await });

        Ok(())
    }
    /// Function to get all RTMP servers
    ///
    /// # Returns
    /// * `HashMap<u16, String>` - The servers address
    ///
    pub async fn get_all_rtmp_servers(&self) -> HashMap<u16, std::string::String> {
        self.servers.read().await.clone()
    
    }
    /// Function to get RTMP servers by ID
    ///
    /// # Arguments
    /// * `id` - The ID of the server
    ///
    /// # Returns
    /// * `String` - The server address
    ///
    pub fn get_by_id_rtmp_servers(&self, id: u16) -> String {
        self.servers.lock().unwrap().get(&id).unwrap().clone()
    }
}
