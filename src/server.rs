pub mod service {
  extern crate actix_web;
  use actix_web::{HttpServer, App, web, HttpRequest, HttpResponse};
  pub use crate::config::conf;

  pub fn start(config: conf::Config) {

    let address = format!("{}:{}", config.service.host, config.service.port);

    println!("listening on address {}", address);

    match HttpServer::new(|| App::new().service(
      web::resource("/").route(web::get().to_async(index))))
      .bind(address)
      .unwrap()
      .run() {
          Ok(()) => println!("called !"),
          Err(e) => println!(" error {}", e),
      };
  }

  fn index(_req: HttpRequest) -> HttpResponse  {
    HttpResponse::Ok().json("Hello rustacean!")
  }
}
