use heck::ToSnakeCase;
use sqlx::prelude::Type;
use strum::Display;
use strum::EnumString;

#[derive(Debug, Display, EnumString, PartialEq, Type)]
#[sqlx(type_name = "text")]
pub enum EventType {
    #[sqlx(rename = "user_created")]
    #[strum(serialize = "user_created")]
    UserCreated,
}

impl From<EventType> for sea_query::Value {
    fn from(t: EventType) -> Self {
        t.to_string().to_snake_case().into()
    }
}

impl From<String> for EventType {
    fn from(s: String) -> Self {
        s.parse().expect("Invalid event type")
    }
}
