use tokio::net::TcpListener;

use crate::router::init_route;

pub async fn init_server() {
    let app = init_route();
    let listener = TcpListener::bind("0.0.0.0:7878").await.unwrap();

    println!("Server initialization complete. Listening on http://0.0.0.0:7878");

    axum::serve(listener, app).await.unwrap();
}
