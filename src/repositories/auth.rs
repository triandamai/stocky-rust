use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::AppState;
use crate::entity::{user_credential, user_login};

#[derive(Debug, Clone)]
pub struct AuthRepository {
     db_conn: DatabaseConnection,
     jwt_secret: String,
}

impl AuthRepository {
    pub fn init(app_state: &AppState)->AuthRepository{
        let state = app_state.clone();
        AuthRepository{
            db_conn:state.db,
            jwt_secret:state.secret
        }
    }

    pub async fn get_user_by_email(
        &self,
        email:&String
    )->Option<user_credential::Model>{
        let find_user = user_credential::Entity::find()
            .filter(user_credential::Column::Email.eq(email))
            .one(&self.db_conn)
            .await;
        match find_user {
            Ok(user_credential) => {
                if user_credential.is_none() {
                    return None
                }
                return Some(user_credential.unwrap())
            }
            Err(_) => None
        }
    }

    pub async fn get_current_sign_in(
        &self,
        user_id: String,
    ) -> Option<(user_login::Model, Option<user_credential::Model>)> {
        let current_sign_in = user_login::Entity::find()
            .filter(user_login::Column::UserId.eq(user_id))
            .find_also_related(user_credential::Entity)
            .one(&self.db_conn)
            .await;

        let result = match current_sign_in {
            Ok(v) => v,
            Err(_) => None,
        };

        result
    }
}
