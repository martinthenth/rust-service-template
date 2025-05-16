use heck::ToSnakeCase;
use sqlx::prelude::Type;

#[derive(Debug, PartialEq, Type)]
#[sqlx(type_name = "text")]
pub enum EventType {
    #[sqlx(rename = "user_created")]
    UserCreated,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<EventType> for sea_query::Value {
    fn from(t: EventType) -> Self {
        t.to_string().to_snake_case().into()
    }
}
