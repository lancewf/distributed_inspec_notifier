pub mod service {
  use warp::Filter;
  pub use crate::config::conf;
  use std::net::ToSocketAddrs;

  #[tokio::main]
  pub async fn start(config: conf::Config) {

    let address = format!("{}:{}", config.service.host, config.service.port);

    println!("listening on address {}", address);

    let root = warp::any()
      .map(|| format!("Hello rustacean!"));

    let mut addr_itr = address.to_socket_addrs()
      .expect("host and port were not correct");

    if let Some(socket_addr) = addr_itr.next() {
      warp::serve(root)
          .run(socket_addr)
          .await;
    }
  }
}
