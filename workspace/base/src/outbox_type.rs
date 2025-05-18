use sqlx::prelude::Type;
use strum::Display;

#[derive(Debug, Display, PartialEq, Type)]
#[sqlx(type_name = "text")]
pub enum OutboxType {
    #[sqlx(rename = "user_created")]
    #[strum(serialize = "user_created")]
    UserCreated,
}
