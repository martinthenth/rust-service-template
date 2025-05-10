use web::web_server::WebServer;

#[tokio::main]
async fn main() {
    WebServer::start_server().await;
}
