use axum::Extension;
use axum::{
    extract::ws::WebSocketUpgrade, http::StatusCode, response::Response, routing::get, Router,
};
use std::net::SocketAddr;
use yerpc::axum::handle_ws_rpc;
use yerpc::{rpc, RpcClient, RpcSession};

#[derive(Clone)]
struct Api;

#[rpc(
    all_positional,
    ts_outdir = "typescript/generated",
    openrpc_outdir = "./"
)]
impl Api {
    async fn shout(&self, msg: String) -> String {
        msg.to_uppercase()
    }
    async fn add(&self, a: f32, b: f32) -> f32 {
        a + b
    }
}
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api = Api {};
    let app = Router::new()
        .route("/rpc", get(handler))
        .layer(Extension(api));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    eprintln!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn handler(ws: WebSocketUpgrade, Extension(api): Extension<Api>) -> Response {
    let (client, out_channel) = RpcClient::new();
    let session = RpcSession::new(client, api);
    handle_ws_rpc(ws, out_channel, session).await
}
