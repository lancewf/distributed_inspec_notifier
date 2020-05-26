pub mod service {
  use warp::Filter;
  pub use crate::config::conf;
  use std::net::ToSocketAddrs;
  use serde_derive::Deserialize;

  #[derive(Debug)]
  struct NotUtf8;
  impl warp::reject::Reject for NotUtf8 {}

  #[derive(Deserialize)]
  struct Report {
    report_uuid: String,
    node_name:   String,
    profiles:    Vec<Profile>,
  }
  #[derive(Deserialize)]
  struct Profile {
    name:   String,
    controls: Vec<Control>,
  }

  #[derive(Deserialize)]
  struct Control {
    id:      String,
    title:   String,
    impact:  f32,
    results: Vec<InspecResult>,
  }

  #[derive(Deserialize)]
  struct InspecResult {
    status:   String, //passed, skipped, failed
    code_desc: String,
  }

  impl Report {
    pub fn print_all(self) {
      println!("report - name: '{}' id: '{}'", self.node_name, self.report_uuid);

      self.profiles.into_iter().for_each(|p| {
        println!("pro - name: {}", p.name);
        p.controls.into_iter().for_each(|c| {
          println!("control - id: {} title: {} impact: {}", c.id, c.title, c.impact);
          c.results.into_iter().for_each(|res| println!("res - status: {} code_desc: {}",
            res.status, res.code_desc))
        })
      });
    }
  }

  #[tokio::main]
  pub async fn start(config: conf::Config) {

    let address = format!("{}:{}", config.service.host, config.service.port);

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
      .map(|body| {
        parse_report(body);
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

  fn parse_report(body: String) {
    let r: Report = serde_json::from_str(&body).expect("message was not parsable");

    r.print_all();
  }
}
