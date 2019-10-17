#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

mod db_ops;
mod db_owner;
mod db_ref;
mod handler;
mod protocol;
mod server;
mod settings;
mod table;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    let sys = actix::System::new("seriesdb-server");
    let db_owner_addr = db_owner::DbOwner::new().run();
    server::run(db_owner_addr);
    sys.run().unwrap();
}
