use std::net::SocketAddr;

use anyhow::Result;
use axum::{response::Json, routing::get, Router};
use serde_json::json;
use tonic::{transport::Server, Request, Response, Status};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use common::AppConfig;
use proto::greeter::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};

#[tokio::main]
async fn main() -> Result<()> {
    common::init_tracing();

    let config = AppConfig::default();
    let http_addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    let grpc_addr: SocketAddr = format!("{}:{}", config.host, config.port + 1).parse()?;

    let http = tokio::spawn(serve_http(http_addr));
    let grpc = tokio::spawn(serve_grpc(grpc_addr));

    tokio::try_join!(flatten(http), flatten(grpc))?;
    Ok(())
}

async fn flatten<T>(handle: tokio::task::JoinHandle<Result<T>>) -> Result<T> {
    match handle.await {
        Ok(res) => res,
        Err(join_err) => Err(anyhow::anyhow!(join_err)),
    }
}

async fn serve_http(addr: SocketAddr) -> Result<()> {
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    tracing::info!(%addr, "http server listening");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn serve_grpc(addr: SocketAddr) -> Result<()> {
    tracing::info!(%addr, "grpc server listening");
    Server::builder()
        .add_service(GreeterServer::new(GreeterService))
        .serve(addr)
        .await?;
    Ok(())
}

async fn root() -> &'static str {
    "rust-devin web service"
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

#[derive(Debug, Default)]
struct GreeterService;

#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> std::result::Result<Response<HelloReply>, Status> {
        let name = request.into_inner().name;
        let reply = HelloReply {
            message: format!("Hello, {}!", name),
        };
        Ok(Response::new(reply))
    }
}
