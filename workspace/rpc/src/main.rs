use rpc::rpc_server::RpcServer;

#[tokio::main]
async fn main() {
    RpcServer::start_server().await;
}
