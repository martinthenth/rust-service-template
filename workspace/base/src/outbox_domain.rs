use heck::ToSnakeCase;
use sqlx::prelude::Type;

#[derive(Debug, PartialEq, Type)]
#[sqlx(type_name = "text")]
pub enum OutboxDomain {
    #[sqlx(rename = "users")]
    Users,
}

impl std::fmt::Display for OutboxDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<OutboxDomain> for sea_query::Value {
    fn from(t: OutboxDomain) -> Self {
        t.to_string().to_snake_case().into()
    }
}
