pub mod server;

pub async fn init_system() {
    server::init_server().await;
}
