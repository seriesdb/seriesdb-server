use crate::db::db_owner::DbOwner;
use crate::db::db_ref::DbRef;
use crate::protocol::{self, *};
use actix::{prelude::*, Addr};
use actix_web_actors::ws;
use bytes::Bytes;
use seriesdb::update::Update as SeriesdbUpdate;
use seriesdb::update_batch::UpdateBatch as SeriesdbUpdateBatch;

pub struct Handler {
    pub db_ref: DbRef<'static>,
}

impl Actor for Handler {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, _ctx: &mut Self::Context) {}
}

impl StreamHandler<ws::Message, ws::ProtocolError> for Handler {
    fn handle(&mut self, ws_msg: ws::Message, ctx: &mut Self::Context) {
        match ws_msg {
            ws::Message::Ping(ws_msg) => {
                ctx.pong(&ws_msg);
            }
            ws::Message::Pong(_) => (),
            ws::Message::Text(text) => ctx.text(text),
            ws::Message::Binary(bin) => {
                let bytes: Bytes = bin.into();
                let res = protocol::decode_from(&bytes);
                match res {
                    Ok(boxed_msg) => self.handle_boxed_msg(boxed_msg, ctx),
                    Err(err) => log::error!("Failed to decode msg: {:?}", err),
                }
            }
            ws::Message::Close(_) => ctx.stop(),
            ws::Message::Nop => (),
        }
    }
}

impl Handler {
    pub fn new(db_owner_addr: Addr<DbOwner>) -> Self {
        Self {
            db_ref: DbRef::new(db_owner_addr),
        }
    }

    fn handle_boxed_msg(&mut self, boxed_msg: BoxedMsg, ctx: &mut <Self as Actor>::Context) {
        match boxed_msg {
            BoxedMsg::PingReq(msg) => {
                ctx.binary(protocol::encode_into(msg));
            }
            BoxedMsg::SetRowsReq(msg) => {
                ctx.binary(protocol::encode_into(self.set_rows(&msg)));
            }
            BoxedMsg::DeleteRowsSinceReq(msg) => {
                ctx.binary(protocol::encode_into(self.delete_rows_since(&msg)));
            }
            BoxedMsg::GetFirstRowReq(msg) => {
                ctx.binary(protocol::encode_into(self.get_first_row(&msg)));
            }
            BoxedMsg::GetLastRowReq(msg) => {
                ctx.binary(protocol::encode_into(self.get_last_row(&msg)));
            }
            BoxedMsg::GetBoundaryRowsReq(msg) => {
                ctx.binary(protocol::encode_into(self.get_boundary_rows(&msg)));
            }
            BoxedMsg::GetRowsSinceReq(msg) => {
                ctx.binary(protocol::encode_into(self.get_rows_since(&msg)));
            }
            BoxedMsg::GetRowsUntilReq(msg) => {
                ctx.binary(protocol::encode_into(self.get_rows_until(&msg)));
            }
            BoxedMsg::GetRowsUntilLastReq(msg) => {
                ctx.binary(protocol::encode_into(self.get_rows_until_last(&msg)));
            }
            BoxedMsg::GetRowsBetweenReq(msg) => {
                ctx.binary(protocol::encode_into(self.get_rows_between(&msg)));
            }
            BoxedMsg::GetFirstKeyReq(msg) => {
                ctx.binary(protocol::encode_into(self.get_first_key(&msg)));
            }
            BoxedMsg::GetLastKeyReq(msg) => {
                ctx.binary(protocol::encode_into(self.get_last_key(&msg)));
            }
            BoxedMsg::GetBoundaryKeysReq(msg) => {
                ctx.binary(protocol::encode_into(self.get_boundary_keys(&msg)));
            }
            BoxedMsg::GetValueReq(msg) => {
                ctx.binary(protocol::encode_into(self.get_value(&msg)));
            }
            BoxedMsg::GetValuesSinceReq(msg) => {
                ctx.binary(protocol::encode_into(self.get_values_since(&msg)));
            }
            BoxedMsg::GetNthLastValueReq(msg) => {
                ctx.binary(protocol::encode_into(self.get_nth_last_value(&msg)));
            }
            BoxedMsg::GetValuesUntilReq(msg) => {
                ctx.binary(protocol::encode_into(self.get_values_until(&msg)));
            }
            BoxedMsg::GetValuesUntilLastReq(msg) => {
                ctx.binary(protocol::encode_into(self.get_values_until_last(&msg)));
            }
            BoxedMsg::GetValuesBetweenReq(msg) => {
                ctx.binary(protocol::encode_into(self.get_values_between(&msg)));
            }
            BoxedMsg::DestroyTableReq(msg) => {
                ctx.binary(protocol::encode_into(self.destroy_table(&msg)));
            }
            BoxedMsg::RenameTableReq(msg) => {
                ctx.binary(protocol::encode_into(self.rename_table(&msg)));
            }
            BoxedMsg::GetTablesReq(msg) => {
                ctx.binary(protocol::encode_into(self.get_tables(&msg)));
            }
            BoxedMsg::GetUpdatesSinceReq(msg) => {
                ctx.binary(protocol::encode_into(self.get_updates_since(&msg)));
            }
            _ => log::error!("Received unknown msg: {:?}", boxed_msg),
        }
    }

    #[inline]
    fn set_rows(&mut self, msg: &SetRowsReq) -> SetRowsRep {
        self.db_ref
            .new_table(&msg.table)
            .set_rows(&msg.keys, &msg.values);
        SetRowsRep {
            round_ref: msg.round_ref,
        }
    }

    #[inline]
    fn delete_rows_since(&mut self, msg: &DeleteRowsSinceReq) -> DeleteRowsSinceRep {
        let table = self.db_ref.new_table(&msg.table);
        table.delete_rows_since(&msg.key, msg.limit);
        DeleteRowsSinceRep {
            round_ref: msg.round_ref,
        }
    }

    #[inline]
    fn get_first_row(&mut self, msg: &GetFirstRowReq) -> GetFirstRowRep {
        match self.db_ref.new_table(&msg.table).get_first_row() {
            Some((key, value)) => GetFirstRowRep {
                key,
                value,
                round_ref: msg.round_ref,
            },
            None => GetFirstRowRep {
                key: none(),
                value: none(),
                round_ref: msg.round_ref,
            },
        }
    }

    #[inline]
    fn get_last_row(&mut self, msg: &GetLastRowReq) -> GetLastRowRep {
        match self.db_ref.new_table(&msg.table).get_last_row() {
            Some((key, value)) => GetLastRowRep {
                key,
                value,
                round_ref: msg.round_ref,
            },
            None => GetLastRowRep {
                key: none(),
                value: none(),
                round_ref: msg.round_ref,
            },
        }
    }

    #[inline]
    fn get_boundary_rows(&mut self, msg: &GetBoundaryRowsReq) -> GetBoundaryRowsRep {
        let table = self.db_ref.new_table(&msg.table);
        match table.get_boundary_rows() {
            Some((first_key, first_value, last_key, last_value)) => GetBoundaryRowsRep {
                first_key,
                first_value,
                last_key,
                last_value,
                round_ref: msg.round_ref,
            },
            None => GetBoundaryRowsRep {
                first_key: none(),
                first_value: none(),
                last_key: none(),
                last_value: none(),
                round_ref: msg.round_ref,
            },
        }
    }

    #[inline]
    fn get_rows_since(&mut self, msg: &GetRowsSinceReq) -> GetRowsSinceRep {
        let table = self.db_ref.new_table(&msg.table);
        let (keys, values) = table.get_rows_since(&msg.key, msg.limit);
        GetRowsSinceRep {
            keys,
            values,
            round_ref: msg.round_ref,
        }
    }

    #[inline]
    fn get_rows_until(&mut self, msg: &GetRowsUntilReq) -> GetRowsUntilRep {
        let table = self.db_ref.new_table(&msg.table);
        let (keys, values) = table.get_rows_until(&msg.key, msg.limit);
        GetRowsUntilRep {
            keys,
            values,
            round_ref: msg.round_ref,
        }
    }

    #[inline]
    fn get_rows_until_last(&mut self, msg: &GetRowsUntilLastReq) -> GetRowsUntilLastRep {
        let table = self.db_ref.new_table(&msg.table);
        let (keys, values) = table.get_rows_until_last(msg.limit);
        GetRowsUntilLastRep {
            keys,
            values,
            round_ref: msg.round_ref,
        }
    }

    #[inline]
    fn get_rows_between(&mut self, msg: &GetRowsBetweenReq) -> GetRowsBetweenRep {
        let table = self.db_ref.new_table(&msg.table);
        let (keys, values) = table.get_rows_between(&msg.begin_key, &msg.end_key, msg.limit);
        GetRowsBetweenRep {
            keys,
            values,
            round_ref: msg.round_ref,
        }
    }

    #[inline]
    fn get_first_key(&mut self, msg: &GetFirstKeyReq) -> GetFirstKeyRep {
        match self.db_ref.new_table(&msg.table).get_first_key() {
            Some(key) => GetFirstKeyRep {
                key,
                round_ref: msg.round_ref,
            },
            None => GetFirstKeyRep {
                key: none(),
                round_ref: msg.round_ref,
            },
        }
    }

    #[inline]
    fn get_last_key(&mut self, msg: &GetLastKeyReq) -> GetLastKeyRep {
        match self.db_ref.new_table(&msg.table).get_last_key() {
            Some(key) => GetLastKeyRep {
                key,
                round_ref: msg.round_ref,
            },
            None => GetLastKeyRep {
                key: none(),
                round_ref: msg.round_ref,
            },
        }
    }

    #[inline]
    fn get_boundary_keys(&mut self, msg: &GetBoundaryKeysReq) -> GetBoundaryKeysRep {
        let table = self.db_ref.new_table(&msg.table);
        match table.get_boundary_keys() {
            Some((first_key, last_key)) => GetBoundaryKeysRep {
                first_key,
                last_key,
                round_ref: msg.round_ref,
            },
            None => GetBoundaryKeysRep {
                first_key: none(),
                last_key: none(),
                round_ref: msg.round_ref,
            },
        }
    }

    #[inline]
    fn get_value(&mut self, msg: &GetValueReq) -> GetValueRep {
        match self.db_ref.new_table(&msg.table).get_value(&msg.key) {
            Some(value) => GetValueRep {
                value,
                round_ref: msg.round_ref,
            },
            None => GetValueRep {
                value: none(),
                round_ref: msg.round_ref,
            },
        }
    }

    #[inline]
    fn get_nth_last_value(&mut self, msg: &GetNthLastValueReq) -> GetNthLastValueRep {
        match self.db_ref.new_table(&msg.table).get_nth_last_value(msg.n) {
            Some(value) => GetNthLastValueRep {
                value,
                round_ref: msg.round_ref,
            },
            None => GetNthLastValueRep {
                value: none(),
                round_ref: msg.round_ref,
            },
        }
    }

    #[inline]
    fn get_values_since(&mut self, msg: &GetValuesSinceReq) -> GetValuesSinceRep {
        let table = self.db_ref.new_table(&msg.table);
        let values = table.get_values_since(&msg.key, msg.limit);
        GetValuesSinceRep {
            values,
            round_ref: msg.round_ref,
        }
    }

    #[inline]
    fn get_values_until(&mut self, msg: &GetValuesUntilReq) -> GetValuesUntilRep {
        let table = self.db_ref.new_table(&msg.table);
        let values = table.get_values_until(&msg.key, msg.limit);
        GetValuesUntilRep {
            values,
            round_ref: msg.round_ref,
        }
    }

    #[inline]
    fn get_values_until_last(&mut self, msg: &GetValuesUntilLastReq) -> GetValuesUntilLastRep {
        let table = self.db_ref.new_table(&msg.table);
        let values = table.get_values_until_last(msg.limit);
        GetValuesUntilLastRep {
            values,
            round_ref: msg.round_ref,
        }
    }

    #[inline]
    fn get_values_between(&mut self, msg: &GetValuesBetweenReq) -> GetValuesBetweenRep {
        let table = self.db_ref.new_table(&msg.table);
        let values = table.get_values_between(&msg.begin_key, &msg.end_key, msg.limit);
        GetValuesBetweenRep {
            values,
            round_ref: msg.round_ref,
        }
    }

    #[inline]
    fn destroy_table(&self, msg: &DestroyTableReq) -> DestroyTableRep {
        self.db_ref.destroy_table(&msg.table);
        DestroyTableRep {
            round_ref: msg.round_ref,
        }
    }

    #[inline]
    fn rename_table(&self, msg: &RenameTableReq) -> RenameTableRep {
        self.db_ref.rename_table(&msg.old_table, &msg.new_table);
        RenameTableRep {
            round_ref: msg.round_ref,
        }
    }

    #[inline]
    fn get_tables(&self, msg: &GetTablesReq) -> GetTablesRep {
        let (names, ids) = self.db_ref.get_tables();
        GetTablesRep {
            names,
            ids,
            round_ref: msg.round_ref,
        }
    }

    #[inline]
    fn get_updates_since(&self, msg: &GetUpdatesSinceReq) -> GetUpdatesSinceRep {
        let iter = self.db_ref.get_updates_since(msg.sn);
        let mut update_batches = Vec::new();
        for sub in iter {
            update_batches.push(Handler::seriesdb_update_batch_to_update_batch(&sub))
        }
        GetUpdatesSinceRep {
            update_batches,
            round_ref: msg.round_ref,
        }
    }

    fn seriesdb_update_batch_to_update_batch(sub: &SeriesdbUpdateBatch) -> UpdateBatch {
        let mut updates = Vec::new();
        for su in &sub.updates {
            updates.push(UpdateWrapper {
                update: Some(Handler::seriesdb_update_to_update(&su)),
            });
        }
        UpdateBatch {
            sn: sub.sn,
            updates,
        }
    }
    fn seriesdb_update_to_update(su: &SeriesdbUpdate) -> update_wrapper::Update {
        match su {
            SeriesdbUpdate::Put { key, value } => {
                update_wrapper::Update::Put(update_wrapper::Put {
                    key: key.to_vec(),
                    value: value.to_vec(),
                })
            }
            SeriesdbUpdate::Delete { key } => {
                update_wrapper::Update::Delete(update_wrapper::Delete { key: key.to_vec() })
            }
            SeriesdbUpdate::DeleteRange { from_key, to_key } => {
                update_wrapper::Update::DeleteRange(update_wrapper::DeleteRange {
                    from_key: from_key.to_vec(),
                    to_key: to_key.to_vec(),
                })
            }
        }
    }
}

#[inline]
fn none() -> Vec<u8> {
    vec![]
}
