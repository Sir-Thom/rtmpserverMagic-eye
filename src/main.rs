use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use api::controller::controller_rtmp::{
    create_rtmp_server_handler, get_all_rtmp_servers_handler, get_by_id_rtmp_servers_handler,delete_rtmp_server_handler
};
use api::service::rtmp_server::RtmpServerManager;
use log::info;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::builder().filter_level(log::LevelFilter::Info).init();
    println!("API is running on: http://127.0.0.1:3030/rtmp");
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(RtmpServerManager::new()))
            .service(
                web::resource("/rtmp/create_rtmp_server/{num_servers}")
                    .to(create_rtmp_server_handler))
            
            .service(web::resource("/rtmp").to(get_all_rtmp_servers_handler))
            .service(web::resource("/rtmp/{id}").to(get_by_id_rtmp_servers_handler))
            .service(web::resource("/rtmp/delete/{id}").to(delete_rtmp_server_handler))
          
    })
    .bind(("0.0.0.0", 3030))?
    .run()
    .await?;
    info!("Actix web server stopped");
    Ok(())
}
