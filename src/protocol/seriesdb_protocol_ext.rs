use std::fmt::{Debug, Formatter, Result as FmtResult};
use bytes::{Bytes, BytesMut, BufMut};
use prost::Message;
pub use prost::DecodeError;
use super::seriesdb_protocol::*;

pub enum BoxedMsg {
    PingReq(PingReq),
    PingRep(PingRep),
    SetRowsReq(SetRowsReq),
    SetRowsRep(SetRowsRep),
    DeleteRowsSinceReq(DeleteRowsSinceReq),
    DeleteRowsSinceRep(DeleteRowsSinceRep),
    GetFirstRowReq(GetFirstRowReq),
    GetFirstRowRep(GetFirstRowRep),
    GetLastRowReq(GetLastRowReq),
    GetLastRowRep(GetLastRowRep),
    GetBoundaryRowsReq(GetBoundaryRowsReq),
    GetBoundaryRowsRep(GetBoundaryRowsRep),
    GetRowsSinceReq(GetRowsSinceReq),
    GetRowsSinceRep(GetRowsSinceRep),
    GetRowsUntilReq(GetRowsUntilReq),
    GetRowsUntilRep(GetRowsUntilRep),
    GetRowsUntilLastReq(GetRowsUntilLastReq),
    GetRowsUntilLastRep(GetRowsUntilLastRep),
    GetRowsBetweenReq(GetRowsBetweenReq),
    GetRowsBetweenRep(GetRowsBetweenRep),
    GetFirstKeyReq(GetFirstKeyReq),
    GetFirstKeyRep(GetFirstKeyRep),
    GetLastKeyReq(GetLastKeyReq),
    GetLastKeyRep(GetLastKeyRep),
    GetBoundaryKeysReq(GetBoundaryKeysReq),
    GetBoundaryKeysRep(GetBoundaryKeysRep),
    GetValueReq(GetValueReq),
    GetValueRep(GetValueRep),
    GetNthLastValueReq(GetNthLastValueReq),
    GetNthLastValueRep(GetNthLastValueRep),
    GetValuesSinceReq(GetValuesSinceReq),
    GetValuesSinceRep(GetValuesSinceRep),
    GetValuesUntilReq(GetValuesUntilReq),
    GetValuesUntilRep(GetValuesUntilRep),
    GetValuesUntilLastReq(GetValuesUntilLastReq),
    GetValuesUntilLastRep(GetValuesUntilLastRep),
    GetValuesBetweenReq(GetValuesBetweenReq),
    GetValuesBetweenRep(GetValuesBetweenRep),
    DestroyTableReq(DestroyTableReq),
    DestroyTableRep(DestroyTableRep),
    RenameTableReq(RenameTableReq),
    RenameTableRep(RenameTableRep),
    GetTablesReq(GetTablesReq),
    GetTablesRep(GetTablesRep),
    GetUpdatesSinceReq(GetUpdatesSinceReq),
    GetUpdatesSinceRep(GetUpdatesSinceRep),
    OkRep(OkRep),
    ErrorRep(ErrorRep),
}

impl Debug for BoxedMsg {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            BoxedMsg::PingReq(msg) => {
                write!(f, "PingReq: {{{:?}}}", msg)
            }
            BoxedMsg::PingRep(msg) => {
                write!(f, "PingRep: {{{:?}}}", msg)
            }
            BoxedMsg::SetRowsReq(msg) => {
                write!(f, "SetRowsReq: {{{:?}}}", msg)
            }
            BoxedMsg::SetRowsRep(msg) => {
                write!(f, "SetRowsRep: {{{:?}}}", msg)
            }
            BoxedMsg::DeleteRowsSinceReq(msg) => {
                write!(f, "DeleteRowsSinceReq: {{{:?}}}", msg)
            }
            BoxedMsg::DeleteRowsSinceRep(msg) => {
                write!(f, "DeleteRowsSinceRep: {{{:?}}}", msg)
            }
            BoxedMsg::GetFirstRowReq(msg) => {
                write!(f, "GetFirstRowReq: {{{:?}}}", msg)
            }
            BoxedMsg::GetFirstRowRep(msg) => {
                write!(f, "GetFirstRowRep: {{{:?}}}", msg)
            }
            BoxedMsg::GetLastRowReq(msg) => {
                write!(f, "GetLastRowReq: {{{:?}}}", msg)
            }
            BoxedMsg::GetLastRowRep(msg) => {
                write!(f, "GetLastRowRep: {{{:?}}}", msg)
            }
            BoxedMsg::GetBoundaryRowsReq(msg) => {
                write!(f, "GetBoundaryRowsReq: {{{:?}}}", msg)
            }
            BoxedMsg::GetBoundaryRowsRep(msg) => {
                write!(f, "GetBoundaryRowsRep: {{{:?}}}", msg)
            }
            BoxedMsg::GetRowsSinceReq(msg) => {
                write!(f, "GetRowsSinceReq: {{{:?}}}", msg)
            }
            BoxedMsg::GetRowsSinceRep(msg) => {
                write!(f, "GetRowsSinceRep: {{{:?}}}", msg)
            }
            BoxedMsg::GetRowsUntilReq(msg) => {
                write!(f, "GetRowsUntilReq: {{{:?}}}", msg)
            }
            BoxedMsg::GetRowsUntilRep(msg) => {
                write!(f, "GetRowsUntilRep: {{{:?}}}", msg)
            }
            BoxedMsg::GetRowsUntilLastReq(msg) => {
                write!(f, "GetRowsUntilLastReq: {{{:?}}}", msg)
            }
            BoxedMsg::GetRowsUntilLastRep(msg) => {
                write!(f, "GetRowsUntilLastRep: {{{:?}}}", msg)
            }
            BoxedMsg::GetRowsBetweenReq(msg) => {
                write!(f, "GetRowsBetweenReq: {{{:?}}}", msg)
            }
            BoxedMsg::GetRowsBetweenRep(msg) => {
                write!(f, "GetRowsBetweenRep: {{{:?}}}", msg)
            }
            BoxedMsg::GetFirstKeyReq(msg) => {
                write!(f, "GetFirstKeyReq: {{{:?}}}", msg)
            }
            BoxedMsg::GetFirstKeyRep(msg) => {
                write!(f, "GetFirstKeyRep: {{{:?}}}", msg)
            }
            BoxedMsg::GetLastKeyReq(msg) => {
                write!(f, "GetLastKeyReq: {{{:?}}}", msg)
            }
            BoxedMsg::GetLastKeyRep(msg) => {
                write!(f, "GetLastKeyRep: {{{:?}}}", msg)
            }
            BoxedMsg::GetBoundaryKeysReq(msg) => {
                write!(f, "GetBoundaryKeysReq: {{{:?}}}", msg)
            }
            BoxedMsg::GetBoundaryKeysRep(msg) => {
                write!(f, "GetBoundaryKeysRep: {{{:?}}}", msg)
            }
            BoxedMsg::GetValueReq(msg) => {
                write!(f, "GetValueReq: {{{:?}}}", msg)
            }
            BoxedMsg::GetValueRep(msg) => {
                write!(f, "GetValueRep: {{{:?}}}", msg)
            }
            BoxedMsg::GetNthLastValueReq(msg) => {
                write!(f, "GetNthLastValueReq: {{{:?}}}", msg)
            }
            BoxedMsg::GetNthLastValueRep(msg) => {
                write!(f, "GetNthLastValueRep: {{{:?}}}", msg)
            }
            BoxedMsg::GetValuesSinceReq(msg) => {
                write!(f, "GetValuesSinceReq: {{{:?}}}", msg)
            }
            BoxedMsg::GetValuesSinceRep(msg) => {
                write!(f, "GetValuesSinceRep: {{{:?}}}", msg)
            }
            BoxedMsg::GetValuesUntilReq(msg) => {
                write!(f, "GetValuesUntilReq: {{{:?}}}", msg)
            }
            BoxedMsg::GetValuesUntilRep(msg) => {
                write!(f, "GetValuesUntilRep: {{{:?}}}", msg)
            }
            BoxedMsg::GetValuesUntilLastReq(msg) => {
                write!(f, "GetValuesUntilLastReq: {{{:?}}}", msg)
            }
            BoxedMsg::GetValuesUntilLastRep(msg) => {
                write!(f, "GetValuesUntilLastRep: {{{:?}}}", msg)
            }
            BoxedMsg::GetValuesBetweenReq(msg) => {
                write!(f, "GetValuesBetweenReq: {{{:?}}}", msg)
            }
            BoxedMsg::GetValuesBetweenRep(msg) => {
                write!(f, "GetValuesBetweenRep: {{{:?}}}", msg)
            }
            BoxedMsg::DestroyTableReq(msg) => {
                write!(f, "DestroyTableReq: {{{:?}}}", msg)
            }
            BoxedMsg::DestroyTableRep(msg) => {
                write!(f, "DestroyTableRep: {{{:?}}}", msg)
            }
            BoxedMsg::RenameTableReq(msg) => {
                write!(f, "RenameTableReq: {{{:?}}}", msg)
            }
            BoxedMsg::RenameTableRep(msg) => {
                write!(f, "RenameTableRep: {{{:?}}}", msg)
            }
            BoxedMsg::GetTablesReq(msg) => {
                write!(f, "GetTablesReq: {{{:?}}}", msg)
            }
            BoxedMsg::GetTablesRep(msg) => {
                write!(f, "GetTablesRep: {{{:?}}}", msg)
            }
            BoxedMsg::GetUpdatesSinceReq(msg) => {
                write!(f, "GetUpdatesSinceReq: {{{:?}}}", msg)
            }
            BoxedMsg::GetUpdatesSinceRep(msg) => {
                write!(f, "GetUpdatesSinceRep: {{{:?}}}", msg)
            }
            BoxedMsg::OkRep(msg) => {
                write!(f, "OkRep: {{{:?}}}", msg)
            }
            BoxedMsg::ErrorRep(msg) => {
                write!(f, "ErrorRep: {{{:?}}}", msg)
            }
        }
    }
}

pub trait EncodeInto: Message + Sized {
    fn encode_type(bytes: &mut BytesMut);

    fn encode_into(&self) -> Bytes {
        let size = self.encoded_len() as usize;
        let mut bytes = BytesMut::with_capacity(size + 1);
        Self::encode_type(&mut bytes);
        if let Err(err) = self.encode(&mut bytes) {
            panic!("Failed to encode msg: {:?}", err);
        }
        return bytes.freeze();
    }
}

impl EncodeInto for PingReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(1);
    }
}

impl EncodeInto for PingRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(2);
    }
}

impl EncodeInto for SetRowsReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(3);
    }
}

impl EncodeInto for SetRowsRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(4);
    }
}

impl EncodeInto for DeleteRowsSinceReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(15);
    }
}

impl EncodeInto for DeleteRowsSinceRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(16);
    }
}

impl EncodeInto for GetFirstRowReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(27);
    }
}

impl EncodeInto for GetFirstRowRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(28);
    }
}

impl EncodeInto for GetLastRowReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(29);
    }
}

impl EncodeInto for GetLastRowRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(30);
    }
}

impl EncodeInto for GetBoundaryRowsReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(35);
    }
}

impl EncodeInto for GetBoundaryRowsRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(36);
    }
}

impl EncodeInto for GetRowsSinceReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(37);
    }
}

impl EncodeInto for GetRowsSinceRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(38);
    }
}

impl EncodeInto for GetRowsUntilReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(39);
    }
}

impl EncodeInto for GetRowsUntilRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(40);
    }
}

impl EncodeInto for GetRowsUntilLastReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(43);
    }
}

impl EncodeInto for GetRowsUntilLastRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(44);
    }
}

impl EncodeInto for GetRowsBetweenReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(45);
    }
}

impl EncodeInto for GetRowsBetweenRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(46);
    }
}

impl EncodeInto for GetFirstKeyReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(47);
    }
}

impl EncodeInto for GetFirstKeyRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(48);
    }
}

impl EncodeInto for GetLastKeyReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(49);
    }
}

impl EncodeInto for GetLastKeyRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(50);
    }
}

impl EncodeInto for GetBoundaryKeysReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(55);
    }
}

impl EncodeInto for GetBoundaryKeysRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(56);
    }
}

impl EncodeInto for GetValueReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(57);
    }
}

impl EncodeInto for GetValueRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(58);
    }
}

impl EncodeInto for GetNthLastValueReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(65);
    }
}

impl EncodeInto for GetNthLastValueRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(66);
    }
}

impl EncodeInto for GetValuesSinceReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(67);
    }
}

impl EncodeInto for GetValuesSinceRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(68);
    }
}

impl EncodeInto for GetValuesUntilReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(69);
    }
}

impl EncodeInto for GetValuesUntilRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(70);
    }
}

impl EncodeInto for GetValuesUntilLastReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(73);
    }
}

impl EncodeInto for GetValuesUntilLastRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(74);
    }
}

impl EncodeInto for GetValuesBetweenReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(75);
    }
}

impl EncodeInto for GetValuesBetweenRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(76);
    }
}

impl EncodeInto for DestroyTableReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(79);
    }
}

impl EncodeInto for DestroyTableRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(80);
    }
}

impl EncodeInto for RenameTableReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(81);
    }
}

impl EncodeInto for RenameTableRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(82);
    }
}

impl EncodeInto for GetTablesReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(83);
    }
}

impl EncodeInto for GetTablesRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(84);
    }
}

impl EncodeInto for GetUpdatesSinceReq {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(90);
    }
}

impl EncodeInto for GetUpdatesSinceRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(91);
    }
}

impl EncodeInto for OkRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(99);
    }
}

impl EncodeInto for ErrorRep {
    fn encode_type(bytes: &mut BytesMut) {
        bytes.put_u8(100);
    }
}

pub fn encode_into<T: EncodeInto>(msg: T) -> Bytes {
    return msg.encode_into()
}

pub fn decode_from(bytes: &Bytes) -> Result<BoxedMsg, DecodeError> {
    let msg_type = bytes[0] as i8;
    let msg_bytes = bytes.slice_from(1);
    if msg_type == 1 {
        let res: Result<PingReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::PingReq(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 2 {
        let res: Result<PingRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::PingRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 3 {
        let res: Result<SetRowsReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::SetRowsReq(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 4 {
        let res: Result<SetRowsRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::SetRowsRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 15 {
        let res: Result<DeleteRowsSinceReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::DeleteRowsSinceReq(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 16 {
        let res: Result<DeleteRowsSinceRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::DeleteRowsSinceRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 27 {
        let res: Result<GetFirstRowReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetFirstRowReq(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 28 {
        let res: Result<GetFirstRowRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetFirstRowRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 29 {
        let res: Result<GetLastRowReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetLastRowReq(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 30 {
        let res: Result<GetLastRowRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetLastRowRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 35 {
        let res: Result<GetBoundaryRowsReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetBoundaryRowsReq(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 36 {
        let res: Result<GetBoundaryRowsRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetBoundaryRowsRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 37 {
        let res: Result<GetRowsSinceReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetRowsSinceReq(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 38 {
        let res: Result<GetRowsSinceRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetRowsSinceRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 39 {
        let res: Result<GetRowsUntilReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetRowsUntilReq(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 40 {
        let res: Result<GetRowsUntilRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetRowsUntilRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 43 {
        let res: Result<GetRowsUntilLastReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetRowsUntilLastReq(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 44 {
        let res: Result<GetRowsUntilLastRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetRowsUntilLastRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 45 {
        let res: Result<GetRowsBetweenReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetRowsBetweenReq(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 46 {
        let res: Result<GetRowsBetweenRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetRowsBetweenRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 47 {
        let res: Result<GetFirstKeyReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetFirstKeyReq(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 48 {
        let res: Result<GetFirstKeyRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetFirstKeyRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 49 {
        let res: Result<GetLastKeyReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetLastKeyReq(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 50 {
        let res: Result<GetLastKeyRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetLastKeyRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 55 {
        let res: Result<GetBoundaryKeysReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetBoundaryKeysReq(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 56 {
        let res: Result<GetBoundaryKeysRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetBoundaryKeysRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 57 {
        let res: Result<GetValueReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetValueReq(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 58 {
        let res: Result<GetValueRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetValueRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 65 {
        let res: Result<GetNthLastValueReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetNthLastValueReq(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 66 {
        let res: Result<GetNthLastValueRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetNthLastValueRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 67 {
        let res: Result<GetValuesSinceReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetValuesSinceReq(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 68 {
        let res: Result<GetValuesSinceRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetValuesSinceRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 69 {
        let res: Result<GetValuesUntilReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetValuesUntilReq(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 70 {
        let res: Result<GetValuesUntilRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetValuesUntilRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 73 {
        let res: Result<GetValuesUntilLastReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetValuesUntilLastReq(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 74 {
        let res: Result<GetValuesUntilLastRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetValuesUntilLastRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 75 {
        let res: Result<GetValuesBetweenReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetValuesBetweenReq(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 76 {
        let res: Result<GetValuesBetweenRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetValuesBetweenRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 79 {
        let res: Result<DestroyTableReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::DestroyTableReq(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 80 {
        let res: Result<DestroyTableRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::DestroyTableRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 81 {
        let res: Result<RenameTableReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::RenameTableReq(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 82 {
        let res: Result<RenameTableRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::RenameTableRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 83 {
        let res: Result<GetTablesReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetTablesReq(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 84 {
        let res: Result<GetTablesRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetTablesRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 90 {
        let res: Result<GetUpdatesSinceReq, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetUpdatesSinceReq(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 91 {
        let res: Result<GetUpdatesSinceRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::GetUpdatesSinceRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 99 {
        let res: Result<OkRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::OkRep(msg)),
            Err(err) => Err(err)
        }
    } else if msg_type == 100 {
        let res: Result<ErrorRep, DecodeError> = Message::decode(msg_bytes);
        match res {
            Ok(msg) => Ok(BoxedMsg::ErrorRep(msg)),
            Err(err) => Err(err)
        }
    } else {
        Err(DecodeError::new(format!("Invalid msg type: {}", msg_type)))
    }
}