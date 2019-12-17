use crate::db::db_ops::*;
use crate::db::table::Table;
use crate::settings::SETTINGS;
use actix::{prelude::*, Actor, Addr, Context};
use seriesdb::{db::Db as InnerDb, options::Options};

lazy_static! {
    pub static ref INNER_DB: InnerDb = { new_inner_db() };
}

fn new_inner_db() -> InnerDb {
    let mut opts = Options::new();
    opts.set_table_cache_num_shard_bits(SETTINGS.table_cache_num_shard_bits);
    opts.set_write_buffer_size(SETTINGS.write_buffer_size);
    opts.set_max_write_buffer_number(SETTINGS.max_write_buffer_number);
    opts.set_min_write_buffer_number_to_merge(SETTINGS.min_write_buffer_number_to_merge);
    opts.set_max_bytes_for_level_base(SETTINGS.max_bytes_for_level_base);
    opts.set_max_bytes_for_level_multiplier(SETTINGS.max_bytes_for_level_multiplier);
    opts.set_target_file_size_base(SETTINGS.target_file_size_base);
    opts.set_target_file_size_multiplier(SETTINGS.target_file_size_multiplier);
    opts.set_level_zero_file_num_compaction_trigger(
        SETTINGS.level_zero_file_num_compaction_trigger,
    );
    opts.set_max_background_compactions(SETTINGS.max_background_compactions);
    opts.set_max_background_flushes(SETTINGS.max_background_flushes);
    InnerDb::new(&SETTINGS.data_dir, &opts).unwrap()
}

pub struct DbOwner;

impl Actor for DbOwner {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("DbOwner actor started.");
    }

    fn stopped(&mut self, _: &mut Context<Self>) {
        log::info!("DbOwner actor stopped.");
    }
}

impl Handler<NewTableReq> for DbOwner {
    type Result = NewTableRep;

    fn handle(&mut self, msg: NewTableReq, _ctx: &mut Context<Self>) -> Self::Result {
        Ok(Table::new(INNER_DB.new_table(&msg.name).unwrap()))
    }
}

// impl Handler<DestroyTableReq> for DbOwner {
//     type Result = DestroyTableRep;

//     fn handle(&mut self, msg: DestroyTableReq, _ctx: &mut Context<Self>) -> Self::Result {
//         INNER_DB.destroy_table(&msg.name).unwrap();
//         Ok(())
//     }
// }

impl Handler<TruncateTableReq> for DbOwner {
    type Result = TruncateTableRep;

    fn handle(&mut self, msg: TruncateTableReq, _ctx: &mut Context<Self>) -> Self::Result {
        INNER_DB.truncate_table(&msg.name).unwrap();
        Ok(())
    }
}

impl Handler<RenameTableReq> for DbOwner {
    type Result = RenameTableRep;

    fn handle(&mut self, msg: RenameTableReq, _ctx: &mut Context<Self>) -> Self::Result {
        INNER_DB.rename_table(&msg.old_name, &msg.new_name).unwrap();
        Ok(())
    }
}

impl Handler<GetTablesReq> for DbOwner {
    type Result = GetTablesRep;

    fn handle(&mut self, _msg: GetTablesReq, _ctx: &mut Context<Self>) -> Self::Result {
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

impl Handler<GetInnerDbReq> for DbOwner {
    type Result = GetInnerDbRep;

    fn handle(&mut self, _msg: GetInnerDbReq, _ctx: &mut Context<Self>) -> Self::Result {
        Ok(&INNER_DB)
    }
}

impl DbOwner {
    pub fn start() -> Addr<Self> {
        let arbiter = Arbiter::new();
        DbOwner::start_in_arbiter(&arbiter, |_ctx| DbOwner)
    }
}
