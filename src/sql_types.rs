use schemars::{JsonSchema};
#[derive(Debug, Clone, Copy, QueryId, SqlType, JsonSchema)]
#[postgres(type_name = "citext")]
pub struct Citext;
