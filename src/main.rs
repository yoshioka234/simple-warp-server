use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use warp::Filter;

#[tokio::main]
async fn main() {
    let srv_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let routes = warp::any().map(|| "Hello, World!");

    warp::serve(routes).run(srv_addr).await;
}
