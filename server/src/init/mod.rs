pub mod server;

pub async fn init_system() {
    println!("System initialization complete.");
    server::init_server().await;
}
