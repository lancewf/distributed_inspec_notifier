pub mod service {
  use warp::Filter;
  pub use crate::model::model;
  pub use crate::config::conf;

  use std::net::ToSocketAddrs;
  use std::convert::Infallible;
  use hyper::{body::HttpBody as _, Client};
  use tokio::io::{self, AsyncWriteExt as _};

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
      .and_then(report_processor);

    let mut addr_itr = address.to_socket_addrs()
      .expect("host and port were not correct");

    let routes = warp::post().and(inspec).or(all_others);

    if let Some(socket_addr) = addr_itr.next() {
      warp::serve(routes)
          .run(socket_addr)
          .await;
    }
  }

  async fn report_processor(body: String, config: std::sync::Arc<conf::Config>) -> Result<impl warp::Reply, Infallible>{
      // forward to automate

      let report = model::Report::from_str(body);
      if report.has_notification_to_send() && config.webhook.url != "" {
        println!("send message to {}", config.webhook.url);
        return match send(config).await {
          Ok(()) => Ok("ok"),
          Err(_) => Ok("error sending notification"),
        }
      }
      Ok("no notification sent")
  }

  async fn send(config: std::sync::Arc<conf::Config>) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {

    // HTTPS requires picking a TLS implementation, so give a better
    // warning if the user tries to request an 'https' URL.
    let url = config.webhook.url.parse::<hyper::Uri>().unwrap();
    if url.scheme_str() != Some("http") {
        println!("This example only works with 'http' URLs.");
        return Ok(());
    }

    let client = Client::new();

    let mut res = client.get(url).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    // Stream the body, writing each chunk to stdout as we get it
    // (instead of buffering and printing at the end).
    while let Some(next) = res.data().await {
        let chunk = next?;
        io::stdout().write_all(&chunk).await?;
    }

    println!("\n\nDone!");

    Ok(())
  }
}
