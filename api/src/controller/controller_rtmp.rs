use crate::service::rtmp_server::RtmpServerManager;
pub use actix_web::{web, HttpResponse, Responder};
pub use log::{error, info, warn};
pub use serde_json::json;
pub use std::collections::HashMap;
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
///
///
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
