use crate::db::table::Table;
use actix::prelude::*;
use seriesdb::db::Db;

pub(in crate) type Errcode = i8;

#[derive(Message)]
#[rtype(NewTableRep)]
#[derive(Debug)]
pub(in crate) struct NewTableReq {
    pub name: String,
}
pub(in crate) type NewTableRep = Result<Table<'static>, Errcode>;

#[derive(Message)]
#[rtype(DestroyTableRep)]
#[derive(Debug)]
pub(in crate) struct DestroyTableReq {
    pub name: String,
}
pub(in crate) type DestroyTableRep = Result<(), Errcode>;

#[derive(Message)]
#[rtype(RenameTableRep)]
#[derive(Debug)]
pub(in crate) struct RenameTableReq {
    pub old_name: String,
    pub new_name: String,
}
pub(in crate) type RenameTableRep = Result<(), Errcode>;

#[derive(Message)]
#[rtype(GetTablesRep)]
#[derive(Debug)]
pub(in crate) struct GetTablesReq;
pub(in crate) type GetTablesRep = Result<(Vec<String>, Vec<u32>), Errcode>;

#[derive(Message)]
#[rtype(GetInnerDbRep)]
#[derive(Debug)]
pub(in crate) struct GetInnerDbReq;
pub(in crate) type GetInnerDbRep = Result<&'static Db, Errcode>;
