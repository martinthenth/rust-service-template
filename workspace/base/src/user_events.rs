use prost::Message;

use crate::database::DbExecutor;
use crate::error::Error;
use crate::outbox::Outbox;
use crate::outbox_type::OutboxType;
use crate::outboxes::CreateOutboxParams;
use crate::outboxes::Outboxes;
use crate::user::User;
use crate::users::events::UserCreated;
use crate::users::types::User as ProtoUser;

pub struct UserEvents;

impl UserEvents {
    pub async fn create_user_created_event(
        db: impl DbExecutor<'_>,
        user: &User,
    ) -> Result<Outbox, Error> {
        let payload = UserCreated {
            user: Some(ProtoUser {
                id: user.id.to_string(),
                first_name: user.first_name.clone(),
                last_name: user.last_name.clone(),
                banned_at: user.banned_at.map_or("".to_string(), |dt| dt.to_string()),
                created_at: user.created_at.to_string(),
                updated_at: user.updated_at.to_string(),
                deleted_at: user.deleted_at.map_or("".to_string(), |dt| dt.to_string()),
            }),
        };
        let params = CreateOutboxParams {
            key: user.id,
            r#type: OutboxType::UserCreated,
            payload: payload.encode_to_vec(),
        };

        Outboxes::create_outbox(db, params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Factory;
    use crate::outbox_topic::OutboxTopic;

    mod create_user_created_event {
        use super::*;
        use crate::common::Envelope;

        #[meta::data_case]
        async fn returns_outbox() {
            let user = User::insert(&mut *conn, User::factory()).await;

            let result = UserEvents::create_user_created_event(&mut *conn, &user)
                .await
                .unwrap();

            assert_eq!(result.topic, OutboxTopic::UsersEvents);
            assert_eq!(result.key, user.id);

            let payload = Envelope::decode(result.payload.as_slice()).unwrap();

            assert_eq!(payload.r#type, OutboxType::UserCreated.to_string());
            assert_eq!(
                payload.payload,
                UserCreated {
                    user: Some(ProtoUser {
                        id: user.id.to_string(),
                        first_name: user.first_name.clone(),
                        last_name: user.last_name.clone(),
                        banned_at: user.banned_at.map_or("".to_string(), |dt| dt.to_string()),
                        created_at: user.created_at.to_string(),
                        updated_at: user.updated_at.to_string(),
                        deleted_at: user.deleted_at.map_or("".to_string(), |dt| dt.to_string()),
                    }),
                }
                .encode_to_vec()
            );
        }
    }
}
