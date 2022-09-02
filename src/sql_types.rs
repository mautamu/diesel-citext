use schemars::{JsonSchema};
#[derive(Debug, Clone, Copy, QueryId, SqlType, JsonSchema)]
#[diesel(postgres_type(name = "citext"))]
pub struct Citext;
