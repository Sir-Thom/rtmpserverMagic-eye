use rtmpserver::api::api::api_main;
fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    api_main().expect("Error starting API");
    Ok(())
}
