use crate::data::structures::header::Header;

pub(super) enum Data {
    Header(Header),
    Timestamp(i64),
    I64(Option<i64>),
    String(Option<String>),
}
