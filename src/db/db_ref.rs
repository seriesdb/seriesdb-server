use crate::db::db_ops::*;
use crate::db::db_owner::DbOwner;
use crate::db::table::Table;
use actix::{prelude::*, Addr};
use seriesdb::db::Db;
use seriesdb::update_iterator::UpdateIterator;
use std::collections::{hash_map::Entry, HashMap};

pub struct DbRef<'a> {
    cache: HashMap<String, Table<'a>>,
    db_owner_addr: Addr<DbOwner>,
    inner_db: &'static Db,
}

impl<'a> DbRef<'a> {
    #[inline]
    pub fn new(db_owner_addr: Addr<DbOwner>) -> DbRef<'a> {
        let req = GetInnerDbReq {};
        let inner_db = &db_owner_addr.send(req).wait().unwrap().unwrap();
        DbRef {
            cache: HashMap::new(),
            db_owner_addr,
            inner_db,
        }
    }

    #[inline]
    pub fn new_table(&mut self, name: &str) -> &Table {
        match self.cache.entry(name.to_owned()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                let req = NewTableReq {
                    name: name.to_owned(),
                };
                v.insert(self.db_owner_addr.send(req).wait().unwrap().unwrap())
            }
        }
    }

    #[inline]
    pub fn destroy_table(&self, name: &str) {
        let req = DestroyTableReq {
            name: name.to_owned(),
        };
        self.db_owner_addr.send(req).wait().unwrap().unwrap();
    }

    #[inline]
    pub fn rename_table(&self, old_name: &str, new_name: &str) {
        let req = RenameTableReq {
            old_name: old_name.to_owned(),
            new_name: new_name.to_owned(),
        };
        self.db_owner_addr.send(req).wait().unwrap().unwrap();
    }

    #[inline]
    pub fn get_tables(&self) -> (Vec<String>, Vec<u32>) {
        let req = GetTablesReq {};
        self.db_owner_addr.send(req).wait().unwrap().unwrap()
    }

    #[inline]
    pub fn get_updates_since(&self, sn: u64) -> UpdateIterator {
        self.inner_db.get_updates_since(sn).unwrap()
    }
}
