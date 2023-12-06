use sea_orm::{DatabaseConnection, EntityTrait};
use crate::entity::user_credential;

#[derive(Debug, Clone)]
pub struct UserRepository {
    pub db_conn: DatabaseConnection,
}

impl UserRepository {
    pub async fn get_users(&self) -> Option<Vec<user_credential::Model>> {
        let data = user_credential::Entity::find()
            .all(&self.db_conn).await;

        if data.is_err() {
            return None;
        }

        return Some(data.unwrap());
    }
}
