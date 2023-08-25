use crate::service::rtmp_server::RtmpServerManager;
pub use actix_web::{web, HttpResponse, Responder};
use lazy_static::lazy_static;
pub use log::{error, info, warn};
pub use serde_json::json;
pub use std::collections::HashMap;
use tokio::sync::RwLock;

// Define a static variable to hold the RTMP servers data.
lazy_static! {
    static ref RTMP_SERVERS: RwLock<HashMap<u16, String>> = RwLock::new(HashMap::new());
}
/// Function to get all RTMP servers
///
/// # Arguments
/// * `server_manager` - The server manager
///
/// # Returns
/// * `impl Responder` - The response
///
/// # Example
/// ```
/// use actix_web::{web, App, HttpServer};
/// use api::controller::controller_rtmp::get_all_rtmp_servers_handler;
/// use api::service::rtmp_server::RtmpServerManager;
///
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///   HttpServer::new(|| {
///     App::new()
///      .app_data(Data::new(RtmpServerManager::new()))
///     .service(web::resource("/rtmp").to(get_all_rtmp_servers_handler))
///  })
/// .bind(("ip", port))?
/// .run()
/// .await?;
/// Ok(())
/// }
/// ```
///
/// #Error
/// Returns an error if no RTMP servers are running
///
pub async fn get_all_rtmp_servers_handler() -> impl Responder {
    let servers = RTMP_SERVERS.read().await.clone();
    info!("all servers: {:?}", RTMP_SERVERS.read().await.clone());
    if servers.is_empty() {
        warn!("No RTMP servers running");
        return HttpResponse::NotFound().body("No RTMP servers running");
    }
    info!("Successfully retrieved all RTMP servers");
    HttpResponse::Ok().json(&servers)
}
/// Function to get RTMP servers by ID
///
/// # Arguments
/// * `server_manager` - The server manager
/// * `id` - The ID of the server
///
/// # Returns
/// * `impl Responder` - The response
///
/// # Example
/// ```
/// use actix_web::{web, App, HttpServer};
/// use api::controller::controller_rtmp::get_by_id_rtmp_servers_handler;
/// use api::service::rtmp_server::RtmpServerManager;
///     
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///    HttpServer::new(|| {
///      App::new()
///        .app_data(Data::new(RtmpServerManager::new()))
///       .service(web::resource("/rtmp/{id}").to(get_by_id_rtmp_servers_handler))
///  })
/// .bind(("ip", port))?
/// .run()
/// .await?;
/// Ok(())
/// }
/// ```
/// #Error
/// Returns an error if no RTMP servers are running or if the ID is invalid
pub async fn get_by_id_rtmp_servers_handler(id: web::Path<u16>) -> impl Responder {
    let id = id.into_inner();
    let servers = RTMP_SERVERS.read().await.clone();
    match servers.get(&id) {
        Some(server) => {
            let i = servers.get(&id).unwrap().clone();
            info!("Successfully created {} RTMP servers", i);
            HttpResponse::Ok().json(server)
        }
        None => {
            warn!("No RTMP servers running");
            HttpResponse::NotFound().body("No RTMP servers running")
        }
    }
}
/// Function to create RTMP servers
///
/// # Arguments
/// * `server_manager` - The server manager
/// * `num_servers` - The number of servers to create
///
/// # Returns
/// * `impl Responder` - The response
///
/// # Example
/// ```
/// use actix_web::{web, App, HttpServer};
/// use api::controller::controller_rtmp::create_rtmp_server_handler;
/// use api::service::rtmp_server::RtmpServerManager;
///
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///    HttpServer::new(|| {
///       App::new()
///          .app_data(Data::new(RtmpServerManager::new()))
///         .service(
///            web::resource("/rtmp/create_rtmp_server/{num_servers}")
///              .to(create_rtmp_server_handler))
///   })
///  .bind(("ip", port))?
/// .run()
/// .await?;
/// Ok(())
/// }
/// ```
/// #Error
/// Returns an error if the number of servers is invalid
///
pub async fn create_rtmp_server_handler(
    server_manager: web::Data<RtmpServerManager>,
    num_servers: web::Path<u16>,
) -> impl Responder {
    let num_servers = num_servers.into_inner();

    match server_manager.create_rtmp_server(num_servers).await {
        Ok(servers) => {
            for (server_id, server_address) in servers {
                RTMP_SERVERS
                    .write()
                    .await
                    .insert(server_id, server_address.clone());
                info!("Successfully created RTMP server {}", server_id);
            }
            HttpResponse::Ok().body(format!("Successfully created {} RTMP servers", num_servers))
        }
        Err(err) => {
            error!("Error creating RTMP servers: {}", err);
            HttpResponse::InternalServerError()
                .body(format!("Error creating RTMP servers: {}", err))
        }
    }
}

pub async fn delete_rtmp_server_handler(
    server_manager: web::Data<RtmpServerManager>,
    id: web::Path<u16>,
) -> impl Responder {
    let id = id.into_inner();

    server_manager.remove_rtmp_server(id).await; // Await the async call

    RTMP_SERVERS.write().await.remove(&id);
    info!("Successfully deleted RTMP server {}", id);

    HttpResponse::Ok().body(format!("Successfully deleted RTMP server {}", id))
}
