use anyhow::Result;
use futures::{
    future::{self, Either},
    pin_mut,
};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::{
    signal::unix::{signal, SignalKind},
    sync::Notify,
};
use tracing::{error, info};
use warp::Filter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let srv_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let routes = warp::any().map(|| "Hello, World!");
    let shutdown_signal = Arc::new(Notify::new());
    let wait_shutdown = shutdown_signal.clone();

    tokio::spawn(async move {
        let notify_shutdown_complete = wait_shutdown.clone();
        warp::serve(routes)
            .bind_with_graceful_shutdown(srv_addr, async move {
                wait_shutdown.notified().await;
            })
            .1
            .await;
        notify_shutdown_complete.notify_one();
    });

    if let Err(e) = shutdown().await {
        error!("{:#}", e);
    }
    info!("shutting down the server");
    shutdown_signal.notify_one();
    shutdown_signal.notified().await;
    info!("shutdown complete");
}

async fn shutdown() -> Result<()> {
    let mut terminate = signal(SignalKind::terminate())?;
    let terminate = terminate.recv();

    let mut interrupt = signal(SignalKind::interrupt())?;
    let interrupt = interrupt.recv();

    pin_mut!(terminate, interrupt);

    match future::select(terminate, interrupt).await {
        Either::Left(_) => info!("SIGTERM received"),
        Either::Right(_) => info!("SIGINT received"),
    }

    Ok(())
}
