use heck::ToSnakeCase;
use sqlx::prelude::Type;

#[derive(Debug, PartialEq, Type)]
#[sqlx(type_name = "text")]
pub enum EventDomain {
    #[sqlx(rename = "users")]
    Users,
}

impl std::fmt::Display for EventDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<EventDomain> for sea_query::Value {
    fn from(t: EventDomain) -> Self {
        t.to_string().to_snake_case().into()
    }
}
