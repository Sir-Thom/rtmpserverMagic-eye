pub use actix_web::{web, HttpResponse, Responder};
pub use log::{error, info, warn};
pub use rtmp::rtmp::RtmpServer;

pub use serde_json::json;
pub use std::collections::HashMap;
pub use std::env;
use std::sync::RwLock;
pub use std::sync::{Arc, Mutex};
use streamhub::StreamsHub;

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
    servers: Arc<Mutex<HashMap<u16, String>>>,
    server_id_counter: Mutex<u16>, // The server ID counter is now inside the RtmpServerManager
    dynamic_ports: RwLock<Vec<u16>>,
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
            servers: Arc::new(Mutex::new(HashMap::new())),
            server_id_counter: Mutex::new(0),
            dynamic_ports: RwLock::new(Vec::new()),
        }
    }

    fn get_next_dynamic_port(&self) -> u16 {
        let mut dynamic_ports = self.dynamic_ports.write().unwrap();
        for port in 1935..=65535 {
            if !dynamic_ports.contains(&port) {
                dynamic_ports.push(port);
                return port;
            }
        }
        // Fallback to a default port if no available ports are found
        1935
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
    pub async fn create_rtmp_server(&self, num_servers: u16) -> anyhow::Result<Vec<(u16, String)>> {
        let mut stream_hub = StreamsHub::new(None);
        let sender = stream_hub.get_hub_event_sender();

        let mut server_addresses = Vec::new();

        for _ in 0..num_servers {
            let server_id = {
                let mut counter = self.server_id_counter.lock().unwrap();
                let id = *counter;
                *counter += 1;
                id
            };

            let base_ip = env::var("BASE_IP").unwrap_or_else(|_| "127.0.0.1".to_string());
            let ip = format!("{}", base_ip);
            let port = self.get_next_dynamic_port(); // Use the dynamically assigned port
            let address = format!("{ip}:{port}", ip = ip, port = port);

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

    /// Function to get all RTMP servers
    ///
    /// # Returns
    /// * `HashMap<u16, String>` - The servers address
    ///
    pub fn get_all_rtmp_servers(&self) -> HashMap<u16, String> {
        self.servers.lock().unwrap().clone()
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
        info!("{:?}", self.servers.lock().unwrap().get(&id).unwrap());
        self.servers
            .lock()
            .unwrap()
            .get(&id)
            .unwrap()
            .clone()
            .to_string()
    }
}
