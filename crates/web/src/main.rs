use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Result;
use axum::response::Json;
use prost_validate::Validator;
use serde_json::json;
use tonic::{transport::Server, Request, Response, Status};

use common::AppConfig;
use proto::greeter::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};
use proto::rest::greeter_rest_router;
use proto::FILE_DESCRIPTOR_SET;

#[tokio::main]
async fn main() -> Result<()> {
    common::init_tracing();

    let config = AppConfig::default();
    let grpc_addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;

    let service = Arc::new(GreeterService);

    serve_grpc(grpc_addr, Arc::clone(&service))
        .await
        .expect("grpc server failed");

    Ok(())
}

async fn flatten<T>(handle: tokio::task::JoinHandle<Result<T>>) -> Result<T> {
    match handle.await {
        Ok(res) => res,
        Err(join_err) => Err(anyhow::anyhow!(join_err)),
    }
}

async fn serve_grpc(addr: SocketAddr, greeter: Arc<GreeterService>) -> Result<()> {
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()?;

    tracing::info!(%addr, "grpc server listening");
    Server::builder()
        .add_routes(greeter_rest_router(Arc::clone(&greeter)).into())
        .add_service(GreeterServer::from_arc(greeter))
        .add_service(reflection_service)
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
        let request = request.into_inner();
        request
            .validate()
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        let reply = HelloReply {
            message: format!("Hello, {}!", request.name),
        };
        Ok(Response::new(reply))
    }
}
