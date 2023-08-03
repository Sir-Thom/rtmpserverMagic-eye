use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use controller::controller_rtmp::{
    create_rtmp_server_handler, get_all_rtmp_servers_handler, get_by_id_rtmp_servers_handler,
};
use log::info;
use service::rtmp_server::RtmpServerManager;

use super::{controller, service};

#[actix_web::main]
pub async fn api_main() -> std::io::Result<()> {
    println!("API is running on: http://127.0.0.1:3030");
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(RtmpServerManager::new()))
            .service(web::resource("/rtmp/rtmp_servers/").to(get_by_id_rtmp_servers_handler))
            .service(
                web::resource("/rtmp/rtmp_servers/create_rtmp_server/{num_servers}")
                    .to(create_rtmp_server_handler),
            )
            .route(
                "/rtmp/rtmp_servers/",
                web::get().to(get_all_rtmp_servers_handler),
            )
    })
    .bind(("127.0.0.1", 3030))?
    .run()
    .await?;
    info!("Actix web server stopped");
    Ok(())
}
