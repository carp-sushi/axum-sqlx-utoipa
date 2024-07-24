use crate::config::Config;
use std::net::SocketAddr;
use tokio::net::TcpListener;

impl Config {
    pub async fn tcp_listener(&self) -> TcpListener {
        let addr: SocketAddr = self
            .listen_addr
            .parse()
            .expect("Failed to parse listen address");

        TcpListener::bind(addr)
            .await
            .expect("Failed to bind tcp listener")
    }
}
