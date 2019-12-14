// msg type defs 

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingReq {
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingRep {
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRowsReq {
    #[prost(string, tag="1")]
    pub table: std::string::String,
    #[prost(bytes, repeated, tag="2")]
    pub keys: ::std::vec::Vec<std::vec::Vec<u8>>,
    #[prost(bytes, repeated, tag="3")]
    pub values: ::std::vec::Vec<std::vec::Vec<u8>>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRowsRep {
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRowsSinceReq {
    #[prost(string, tag="1")]
    pub table: std::string::String,
    #[prost(bytes, tag="2")]
    pub key: std::vec::Vec<u8>,
    #[prost(uint32, tag="3")]
    pub limit: u32,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRowsSinceRep {
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFirstRowReq {
    #[prost(string, tag="1")]
    pub table: std::string::String,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFirstRowRep {
    #[prost(bytes, tag="1")]
    pub key: std::vec::Vec<u8>,
    #[prost(bytes, tag="2")]
    pub value: std::vec::Vec<u8>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastRowReq {
    #[prost(string, tag="1")]
    pub table: std::string::String,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastRowRep {
    #[prost(bytes, tag="1")]
    pub key: std::vec::Vec<u8>,
    #[prost(bytes, tag="2")]
    pub value: std::vec::Vec<u8>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBoundaryRowsReq {
    #[prost(string, tag="1")]
    pub table: std::string::String,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBoundaryRowsRep {
    #[prost(bytes, tag="1")]
    pub first_key: std::vec::Vec<u8>,
    #[prost(bytes, tag="2")]
    pub first_value: std::vec::Vec<u8>,
    #[prost(bytes, tag="3")]
    pub last_key: std::vec::Vec<u8>,
    #[prost(bytes, tag="4")]
    pub last_value: std::vec::Vec<u8>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRowsSinceReq {
    #[prost(string, tag="1")]
    pub table: std::string::String,
    #[prost(bytes, tag="2")]
    pub key: std::vec::Vec<u8>,
    #[prost(uint32, tag="3")]
    pub limit: u32,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRowsSinceRep {
    #[prost(bytes, repeated, tag="1")]
    pub keys: ::std::vec::Vec<std::vec::Vec<u8>>,
    #[prost(bytes, repeated, tag="2")]
    pub values: ::std::vec::Vec<std::vec::Vec<u8>>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRowsUntilLastReq {
    #[prost(string, tag="1")]
    pub table: std::string::String,
    #[prost(uint32, tag="2")]
    pub limit: u32,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRowsUntilLastRep {
    #[prost(bytes, repeated, tag="1")]
    pub keys: ::std::vec::Vec<std::vec::Vec<u8>>,
    #[prost(bytes, repeated, tag="2")]
    pub values: ::std::vec::Vec<std::vec::Vec<u8>>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRowsUntilReq {
    #[prost(string, tag="1")]
    pub table: std::string::String,
    #[prost(bytes, tag="2")]
    pub key: std::vec::Vec<u8>,
    #[prost(uint32, tag="3")]
    pub limit: u32,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRowsUntilRep {
    #[prost(bytes, repeated, tag="1")]
    pub keys: ::std::vec::Vec<std::vec::Vec<u8>>,
    #[prost(bytes, repeated, tag="2")]
    pub values: ::std::vec::Vec<std::vec::Vec<u8>>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRowsBetweenReq {
    #[prost(string, tag="1")]
    pub table: std::string::String,
    #[prost(bytes, tag="2")]
    pub begin_key: std::vec::Vec<u8>,
    #[prost(bytes, tag="3")]
    pub end_key: std::vec::Vec<u8>,
    #[prost(uint32, tag="4")]
    pub limit: u32,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRowsBetweenRep {
    #[prost(bytes, repeated, tag="1")]
    pub keys: ::std::vec::Vec<std::vec::Vec<u8>>,
    #[prost(bytes, repeated, tag="2")]
    pub values: ::std::vec::Vec<std::vec::Vec<u8>>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFirstKeyReq {
    #[prost(string, tag="1")]
    pub table: std::string::String,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFirstKeyRep {
    #[prost(bytes, tag="1")]
    pub key: std::vec::Vec<u8>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastKeyReq {
    #[prost(string, tag="1")]
    pub table: std::string::String,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastKeyRep {
    #[prost(bytes, tag="1")]
    pub key: std::vec::Vec<u8>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBoundaryKeysReq {
    #[prost(string, tag="1")]
    pub table: std::string::String,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBoundaryKeysRep {
    #[prost(bytes, tag="1")]
    pub first_key: std::vec::Vec<u8>,
    #[prost(bytes, tag="2")]
    pub last_key: std::vec::Vec<u8>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValueReq {
    #[prost(string, tag="1")]
    pub table: std::string::String,
    #[prost(bytes, tag="2")]
    pub key: std::vec::Vec<u8>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValueRep {
    #[prost(bytes, tag="1")]
    pub value: std::vec::Vec<u8>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNthLastValueReq {
    #[prost(string, tag="1")]
    pub table: std::string::String,
    #[prost(uint32, tag="2")]
    pub n: u32,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNthLastValueRep {
    #[prost(bytes, tag="1")]
    pub value: std::vec::Vec<u8>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValuesSinceReq {
    #[prost(string, tag="1")]
    pub table: std::string::String,
    #[prost(bytes, tag="2")]
    pub key: std::vec::Vec<u8>,
    #[prost(uint32, tag="3")]
    pub limit: u32,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValuesSinceRep {
    #[prost(bytes, repeated, tag="1")]
    pub values: ::std::vec::Vec<std::vec::Vec<u8>>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValuesUntilLastReq {
    #[prost(string, tag="1")]
    pub table: std::string::String,
    #[prost(uint32, tag="2")]
    pub limit: u32,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValuesUntilLastRep {
    #[prost(bytes, repeated, tag="1")]
    pub values: ::std::vec::Vec<std::vec::Vec<u8>>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValuesUntilReq {
    #[prost(string, tag="1")]
    pub table: std::string::String,
    #[prost(bytes, tag="2")]
    pub key: std::vec::Vec<u8>,
    #[prost(uint32, tag="3")]
    pub limit: u32,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValuesUntilRep {
    #[prost(bytes, repeated, tag="1")]
    pub values: ::std::vec::Vec<std::vec::Vec<u8>>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValuesBetweenReq {
    #[prost(string, tag="1")]
    pub table: std::string::String,
    #[prost(bytes, tag="2")]
    pub begin_key: std::vec::Vec<u8>,
    #[prost(bytes, tag="3")]
    pub end_key: std::vec::Vec<u8>,
    #[prost(uint32, tag="4")]
    pub limit: u32,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValuesBetweenRep {
    #[prost(bytes, repeated, tag="1")]
    pub values: ::std::vec::Vec<std::vec::Vec<u8>>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestroyTableReq {
    #[prost(string, tag="1")]
    pub table: std::string::String,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestroyTableRep {
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameTableReq {
    #[prost(string, tag="1")]
    pub old_table: std::string::String,
    #[prost(string, tag="2")]
    pub new_table: std::string::String,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameTableRep {
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTablesReq {
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTablesRep {
    #[prost(string, repeated, tag="1")]
    pub names: ::std::vec::Vec<std::string::String>,
    #[prost(uint32, repeated, tag="2")]
    pub ids: ::std::vec::Vec<u32>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUpdatesSinceReq {
    #[prost(uint64, tag="1")]
    pub sn: u64,
    #[prost(uint32, tag="2")]
    pub limit: u32,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUpdatesSinceRep {
    #[prost(message, repeated, tag="1")]
    pub update_batches: ::std::vec::Vec<UpdateBatch>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OkRep {
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorRep {
    #[prost(int32, tag="1")]
    pub code: i32,
    #[prost(string, tag="2")]
    pub desc: std::string::String,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
// data type defs 

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBatch {
    #[prost(uint64, tag="1")]
    pub sn: u64,
    #[prost(message, repeated, tag="2")]
    pub updates: ::std::vec::Vec<UpdateWrapper>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWrapper {
    #[prost(oneof="update_wrapper::Update", tags="1, 2, 3")]
    pub update: ::std::option::Option<update_wrapper::Update>,
}
pub mod update_wrapper {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Put {
        #[prost(bytes, tag="1")]
        pub key: std::vec::Vec<u8>,
        #[prost(bytes, tag="2")]
        pub value: std::vec::Vec<u8>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Delete {
        #[prost(bytes, tag="1")]
        pub key: std::vec::Vec<u8>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeleteRange {
        #[prost(bytes, tag="1")]
        pub from_key: std::vec::Vec<u8>,
        #[prost(bytes, tag="2")]
        pub to_key: std::vec::Vec<u8>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Update {
        #[prost(message, tag="1")]
        Put(Put),
        #[prost(message, tag="2")]
        Delete(Delete),
        #[prost(message, tag="3")]
        DeleteRange(DeleteRange),
    }
}
// msg type enum 

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MsgType {
    /// unused, placeholder for gpb: 0 
    Unknown = 0,
    PingReq = 1,
    PingRep = 2,
    SetRowsReq = 3,
    SetRowsRep = 4,
    /// DELETE_ROW_REQ : 5;
    /// DELETE_ROW_REP : 6;
    /// DELETE_FIRST_ROW_REQ : 7;
    /// DELETE_FIRST_ROW_REP : 8;
    /// DELETE_LAST_ROW_REQ : 9;
    /// DELETE_LAST_ROW_REP : 10;
    /// DELETE_NTH_ROW_REQ : 11;
    /// DELETE_NTH_ROW_REP : 12;
    /// DELETE_NTH_LAST_ROW_REQ : 13;
    /// DELETE_NTH_LAST_ROW_REP : 14;
    DeleteRowsSinceReq = 15,
    /// DELETE_ROWS_UNTIL_REQ : 17;
    /// DELETE_ROWS_UNTIL_REP : 18;
    /// DELETE_ROWS_SINCE_FIRST_REQ : 19;
    /// DELETE_ROWS_SINCE_FIRST_REP : 20;
    /// DELETE_ROWS_UNTIL_LAST_REQ : 21;
    /// DELETE_ROWS_UNTIL_LAST_REP : 22;
    /// DELETE_ROWS_BETWEEN_REQ : 23;
    /// DELETE_ROWS_BETWEEN_REP : 24;
    DeleteRowsSinceRep = 16,
    /// GET_ROW_REQ : 25;
    /// GET_ROW_REP : 26;
    GetFirstRowReq = 27,
    GetFirstRowRep = 28,
    GetLastRowReq = 29,
    GetLastRowRep = 30,
    /// GET_NTH_ROW_REQ : 31;
    /// GET_NTH_ROW_REP : 32;
    /// GET_NTH_LAST_ROW_REQ : 33;
    /// GET_NTH_LAST_ROW_REP : 34;
    GetBoundaryRowsReq = 35,
    GetBoundaryRowsRep = 36,
    GetRowsSinceReq = 37,
    GetRowsSinceRep = 38,
    GetRowsUntilReq = 39,
    GetRowsUntilRep = 40,
    /// GET_ROWS_SINCE_FIRST_REQ : 41;
    /// GET_ROWS_SINCE_FIRST_REP : 42;
    GetRowsUntilLastReq = 43,
    GetRowsUntilLastRep = 44,
    GetRowsBetweenReq = 45,
    GetRowsBetweenRep = 46,
    GetFirstKeyReq = 47,
    GetFirstKeyRep = 48,
    GetLastKeyReq = 49,
    GetLastKeyRep = 50,
    /// GET_NTH_KEY_REQ : 51;
    /// GET_NTH_KEY_REP : 52;
    /// GET_NTH_LAST_KEY_REQ : 53;
    /// GET_NTH_LAST_KEY_REP : 54;
    GetBoundaryKeysReq = 55,
    GetBoundaryKeysRep = 56,
    GetValueReq = 57,
    GetValueRep = 58,
    /// GET_FIRST_VALUE_REQ : 59;
    /// GET_FIRST_VALUE_REP : 60;
    /// GET_LAST_VALUE_REQ : 61;
    /// GET_LAST_VALUE_REP : 62;
    /// GET_NTH_VALUE_REQ : 63;
    /// GET_NTH_VALUE_REP : 64;
    GetNthLastValueReq = 65,
    GetNthLastValueRep = 66,
    GetValuesSinceReq = 67,
    GetValuesSinceRep = 68,
    GetValuesUntilReq = 69,
    GetValuesUntilRep = 70,
    /// GET_VALUES_SINCE_FIRST_REQ : 71;
    /// GET_VALUES_SINCE_FIRST_REP : 72;
    GetValuesUntilLastReq = 73,
    GetValuesUntilLastRep = 74,
    GetValuesBetweenReq = 75,
    GetValuesBetweenRep = 76,
    // EXIST_REQ : 77;
    // EXIST_REP : 78;

    DestroyTableReq = 79,
    DestroyTableRep = 80,
    RenameTableReq = 81,
    RenameTableRep = 82,
    GetTablesReq = 83,
    GetTablesRep = 84,
    GetUpdatesSinceReq = 90,
    GetUpdatesSinceRep = 91,
    OkRep = 99,
    ErrorRep = 100,
}
