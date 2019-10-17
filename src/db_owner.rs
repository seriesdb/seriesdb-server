use crate::db_ops::*;
use crate::settings::SETTINGS;
use crate::table::Table;
use actix::{prelude::*, Actor, Addr, Arbiter, Context};
use seriesdb::{db::Db as InnerDb, options::Options};

lazy_static! {
    pub static ref INNER_DB: InnerDb = { new_inner_db() };
}

fn new_inner_db() -> InnerDb {
    let mut opts = Options::new();
    opts.set_write_buffer_size(SETTINGS.write_buffer_size);
    opts.set_max_write_buffer_number(SETTINGS.max_write_buffer_number);
    opts.set_min_write_buffer_number_to_merge(SETTINGS.min_write_buffer_number_to_merge);
    opts.set_max_background_compactions(SETTINGS.max_background_compactions);
    opts.set_max_background_flushes(SETTINGS.max_background_flushes);
    InnerDb::new(&SETTINGS.data_dir, &opts).unwrap()
}

pub struct DbOwner;

impl Actor for DbOwner {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("Started db_owner successfully!");
    }
}

impl Handler<NewTableReq> for DbOwner {
    type Result = NewTableRep;

    fn handle(&mut self, msg: NewTableReq, _ctx: &mut Context<Self>) -> Self::Result {
        log::debug!("Received msg: {:?}", msg);
        Ok(Table::new(INNER_DB.new_table(&msg.name).unwrap()))
    }
}

impl Handler<DestroyTableReq> for DbOwner {
    type Result = DestroyTableRep;

    fn handle(&mut self, msg: DestroyTableReq, _ctx: &mut Context<Self>) -> Self::Result {
        log::debug!("Received msg: {:?}", msg);
        INNER_DB.destroy_table(&msg.name).unwrap();
        Ok(())
    }
}

impl Handler<RenameTableReq> for DbOwner {
    type Result = RenameTableRep;

    fn handle(&mut self, msg: RenameTableReq, _ctx: &mut Context<Self>) -> Self::Result {
        log::debug!("Received msg: {:?}", msg);
        INNER_DB.rename_table(&msg.old_name, &msg.new_name).unwrap();
        Ok(())
    }
}

impl Handler<GetTablesReq> for DbOwner {
    type Result = GetTablesRep;

    fn handle(&mut self, msg: GetTablesReq, _ctx: &mut Context<Self>) -> Self::Result {
        log::debug!("Received msg: {:?}", msg);
        let mut names = Vec::new();
        let mut ids = Vec::new();
        let tables = INNER_DB.get_tables();
        for (name, id) in tables {
            names.push(name);
            ids.push(id);
        }
        Ok((names, ids))
    }
}

impl DbOwner {
    pub fn new() -> DbOwner {
        DbOwner
    }

    pub fn run(&self) -> Addr<Self> {
        let arbiter = Arbiter::new();
        DbOwner::start_in_arbiter(&arbiter, |_ctx| DbOwner)
    }
}
