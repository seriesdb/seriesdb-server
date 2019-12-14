#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

mod db;
mod handler;
mod protocol;
mod server;
mod settings;
mod sync;

use crate::db::db_owner;
use crate::settings::SETTINGS;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    let sys = actix::System::new("seriesdb-server");
    if SETTINGS.replication_enabled {
        let _addr = sync::syncer::Syncer::start();
    }
    let db_owner_addr = db_owner::DbOwner::start();
    server::start(db_owner_addr);
    sys.run().unwrap();
}
