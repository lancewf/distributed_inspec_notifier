pub mod service {
  use warp::Filter;
  pub use crate::model::model;
  pub use crate::config::conf;

  use std::net::ToSocketAddrs;
  use std::convert::Infallible;
  use hyper::{Client, Request, Body};

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

  async fn report_processor(body: String, 
    config: std::sync::Arc<conf::Config>) -> Result<impl warp::Reply, Infallible>{
      let report = model::Report::from_str(body);
      if report.has_notification_to_send() {
        if config.webhook.url != "" {
          println!("sending webhook message to {}", config.webhook.url);

          match send(&config.webhook.url, report.webhook_message()).await {
            Ok(()) => (),
            Err(_) => return Ok("error sending webhook notification"),
          }
        }

        if config.slack_webhook.url != "" {
          println!("sending slack message to {}", config.slack_webhook.url);
          match send(&config.slack_webhook.url, report.slack_message()).await {
            Ok(()) => (),
            Err(_) => return Ok("error sending slack notification"),
          }
        }
      }
      Ok("success")
  }



  async fn send(url: &String, body: String) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // HTTPS requires picking a TLS implementation, so give a better
    // warning if the user tries to request an 'https' URL.
    let url = url.parse::<hyper::Uri>().unwrap();
    if url.scheme_str() != Some("http") {
        println!("This example only works with 'http' URLs.");
        return Ok(());
    }

    let client = Client::new();

    let req = Request::builder()
      .method("POST")
      .uri(url)
      .header("Content-Type", "application/json")
      .body(Body::from(body))
      .expect("request builder");

    let res = client.request(req).await?;

    println!("Response: {}", res.status());

    Ok(())
  }
}
