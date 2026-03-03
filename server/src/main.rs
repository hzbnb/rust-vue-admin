mod handle;
mod init;
mod router;

#[tokio::main]
async fn main() {
    init::init_system().await;
}
