use heck::ToSnakeCase;
use sqlx::prelude::Type;

#[derive(Debug, PartialEq, Type)]
#[sqlx(type_name = "text")]
pub enum MessageType {
    #[sqlx(rename = "user_created")]
    UserCreated,
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<MessageType> for sea_query::Value {
    fn from(t: MessageType) -> Self {
        t.to_string().to_snake_case().into()
    }
}
