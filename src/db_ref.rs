use crate::db_ops::*;
use crate::db_owner::DbOwner;
use crate::table::Table;
use actix::{prelude::*, Addr};
use std::collections::{hash_map::Entry, HashMap};

pub struct DbRef<'a> {
    pub cache: HashMap<String, Table<'a>>,
    pub db_owner_addr: Addr<DbOwner>,
}

impl<'a> DbRef<'a> {
    pub fn new(db_owner_addr: Addr<DbOwner>) -> DbRef<'a> {
        DbRef {
            cache: HashMap::new(),
            db_owner_addr,
        }
    }

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

    pub fn destroy_table(&self, name: &str) {
        let req = DestroyTableReq {
            name: name.to_owned(),
        };
        self.db_owner_addr.send(req).wait().unwrap().unwrap();
    }

    pub fn rename_table(&self, old_name: &str, new_name: &str) {
        let req = RenameTableReq {
            old_name: old_name.to_owned(),
            new_name: new_name.to_owned(),
        };
        self.db_owner_addr.send(req).wait().unwrap().unwrap();
    }

    pub fn get_tables(&self) -> (Vec<String>, Vec<u32>) {
        let req = GetTablesReq {};
        self.db_owner_addr.send(req).wait().unwrap().unwrap()
    }
}
