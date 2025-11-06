mod handler;
mod route;

use anyhow::{Context, Result};
use axum::Router;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

use route::v1;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().merge(v1::routes());

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app)
        .await
        .context("Unexpected error occurred in server")
}
