use heck::ToSnakeCase;
use sqlx::prelude::Type;

#[derive(Debug, PartialEq, Type)]
#[sqlx(type_name = "text")]
pub enum OutboxKind {
    #[sqlx(rename = "events")]
    Events,
    #[sqlx(rename = "commands")]
    Commands,
}

impl std::fmt::Display for OutboxKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<OutboxKind> for sea_query::Value {
    fn from(t: OutboxKind) -> Self {
        t.to_string().to_snake_case().into()
    }
}
