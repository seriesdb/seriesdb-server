#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

mod db;
mod handler;
mod protocol;
mod server;
mod settings;
mod table;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    server::run();
}
