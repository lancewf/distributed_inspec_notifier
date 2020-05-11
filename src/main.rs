extern crate actix_web;
use actix_web::{HttpServer, App, web, HttpRequest, HttpResponse};
use std::env;
use std::fs;
use toml;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Config {
    service: Service,
}

#[derive(Deserialize)]
struct Service {
    host: String,
    port: u16,
}

// Here is the handler, 
// we are returning a json response with an ok status 
// that contains the text Hello World
fn index(_req: HttpRequest) -> HttpResponse  {
    HttpResponse::Ok().json("Hello world!")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_file = &args[1];
    println!("config file {}", config_file);
    let file_contents = fs::read_to_string(config_file).unwrap();

    let config: Config = toml::from_str(file_contents.as_str()).unwrap();

    let address = format!("{}:{}", config.service.host, config.service.port);

    println!("listening on address {}", address);

    // We are creating an Application instance and 
    // register the request handler with a route and a resource 
    // that creates a specific path, then the application instance 
    // can be used with HttpServer to listen for incoming connections.
    match HttpServer::new(|| App::new().service(
             web::resource("/").route(web::get().to_async(index))))
        .bind(address)
        .unwrap()
        .run() {
            Ok(()) => println!("called !"),
            Err(e) => println!(" error {}", e),
        };
}
