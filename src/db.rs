use crate::settings::SETTINGS;
use crate::table::Table;
use seriesdb::db::Db as InnerDb;
use seriesdb::options::Options;

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
    InnerDb::new2(&SETTINGS.data_dir, &opts).unwrap()
}

pub struct Db {}

impl Db {
    pub fn new_table(name: &str) -> Table {
        Table::new(INNER_DB.new_table(name).unwrap())
    }

    pub fn destroy_table(name: &str) {
        INNER_DB.destroy_table(name).unwrap();
    }

    pub fn rename_table(old_name: &str, new_name: &str) {
        INNER_DB.rename_table(old_name, new_name).unwrap();
    }

    pub fn get_tables() -> (Vec<String>, Vec<u32>) {
        let mut names = Vec::new();
        let mut ids = Vec::new();
        let tables = INNER_DB.get_tables();
        for (name, id) in tables {
            names.push(name);
            ids.push(id);
        }
        return (names, ids);
    }
}
