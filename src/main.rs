use std::env;
mod server;
pub use crate::server::service;
mod config;
pub use crate::config::conf;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let config_file = &args[1];
        println!("config file {}", config_file);

        let config: conf::Config = conf::get_config(config_file);

        service::start(config);
    } else {
        println!("config file need to be passed as a parameter");
    }
}
