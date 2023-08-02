use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use env_logger::Env;
use log::info;
use rtmpserver::service::rtmp_server::{
    create_rtmp_server_handler, get_all_rtmp_servers_handler, get_by_id_rtmp_servers_handler,
    RtmpServerManager,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    //print api address
    println!("API is running on: http://127.0.0.1:3030");
    // Start the Actix web server
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(RtmpServerManager::new()))
            .service(web::resource("/").to(|| async { "hello world" }))
            .service(web::resource("/rtmp").to(|| async {
                info!("API: Received request for /rtmp");
                "RTMP servers are running!"
            }))
            .service(
                web::resource("/rtmp/rtmp_servers/create_rtmp_server/{num_servers}")
                    .to(create_rtmp_server_handler),
            )
            .route(
                "/rtmp/rtmp_servers/",
                web::get().to(get_all_rtmp_servers_handler),
            )
            .service(web::resource("/rtmp/rtmp_servers/{id}").to(get_by_id_rtmp_servers_handler))
    })
    .bind(("127.0.0.1", 3030))?
    .run()
    .await?;
    info!("Actix web server stopped");
    tokio::signal::ctrl_c().await?;
    info!("Ctrl-C received, shutting down");
    Ok(())
}
