#[derive(Debug, Clone, Copy, QueryId, SqlType)]
#[postgres(type_name = "citext")]
pub struct Citext;
