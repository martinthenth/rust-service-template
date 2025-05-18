use heck::ToSnakeCase;
use sqlx::prelude::Type;
use strum::Display;

#[derive(Debug, Display, PartialEq, Type)]
#[sqlx(type_name = "text")]
pub enum OutboxTopic {
    #[sqlx(rename = "users.events")]
    UsersEvents,
}

impl From<OutboxTopic> for sea_query::Value {
    fn from(t: OutboxTopic) -> Self {
        let snake = t.to_string().to_snake_case();
        if let Some(pos) = snake.rfind('_') {
            let mut s = snake;
            s.replace_range(pos..=pos, ".");
            s.into()
        } else {
            snake.into()
        }
    }
}
