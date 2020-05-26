pub mod service {
  use warp::Filter;
  pub use crate::model::model;
  pub use crate::config::conf;

  use std::net::ToSocketAddrs;

  #[derive(Debug)]
  struct NotUtf8;
  impl warp::reject::Reject for NotUtf8 {}

  #[tokio::main]
  pub async fn start(config: conf::Config) {

    let address = format!("{}:{}", config.service.host, config.service.port);

    let users = std::sync::Arc::new(config);
    let users = warp::any().map(move || users.clone());

    println!("listening on address {}", address);

    let all_others = warp::any()
      .map(|| format!("Hello rustacean!"));

    let inspec = warp::path!("data-collector" / "v0")
      .and(
        warp::body::bytes().and_then(|body: bytes::Bytes| async move {
            std::str::from_utf8(&body)
                .map(String::from)
                .map_err(|_e| warp::reject::custom(NotUtf8))
        }),
      )
      .and(users.clone())
      .map(|body, c| {
        // forward to automate

        let r = model::Report::from_str(body);
        send_notification(r, c);

        Ok("ok")
      });

    let mut addr_itr = address.to_socket_addrs()
      .expect("host and port were not correct");

    let routes = warp::post().and(inspec).or(all_others);

    if let Some(socket_addr) = addr_itr.next() {
      warp::serve(routes)
          .run(socket_addr)
          .await;
    }
  }

  fn send_notification(report: model::Report, config: std::sync::Arc<conf::Config>) {
    if report.has_notification_to_send() {
      if config.webhook.url != "" {
        println!("send message to {}", config.webhook.url);
      }
    }
  }
}
