use std::default::Default as time_default;

use chrono::FixedOffset;
use redis::{Client, ConnectionLike};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QueryFilter};
use sea_orm::ActiveValue::Set;

use crate::AppState;
use crate::entity::{user_credential, user_login, user_verification};
use crate::entity::sea_orm_active_enums::{AuthProvider, Status, VerificationType};

#[derive(Debug, Clone)]
pub struct AuthRepository {
     db: DatabaseConnection,
     cache:Client
}

impl AuthRepository {
    pub fn init(app_state: &AppState)->AuthRepository{
        let state = app_state.clone();
        AuthRepository{
            db:state.db,
            cache:state.cache
        }
    }

    pub async fn is_email_used(
        &self,
        email:&String
    )->bool{
        let find_email = user_credential::Entity::find()
            .filter(user_credential::Column::Email.eq(email))
            .count(&self.db)
            .await;
        match find_email {
            Ok(account) => account > 0,
            Err(_) => false
        }
    }

    pub async fn get_user_by_email(
        &self,
        email:&String
    )->Option<user_credential::Model>{
        let find_user = user_credential::Entity::find()
            .filter(user_credential::Column::Email.eq(email))
            .one(&self.db)
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

    pub async fn insert_user_basic(
        &self,
        email:&String,
        password:&String,
        full_name:&String
    )->Result<user_credential::Model,DbErr>{
        let current_date = chrono::DateTime::<FixedOffset>::default().naive_local();

        let uuid = uuid::Uuid::new_v4();
        let prepare_data = user_credential::ActiveModel{
            id: Set(uuid.to_string()),
            email: Set(email.to_string()),
            full_name: Set(full_name.to_string()),
            password: Set(password.to_string()),
            status: Set(Status::WaitingConfirmation),
            auth_provider: Set(AuthProvider::Basic),
            created_at:Set(current_date),
            updated_at: Set(current_date),
            deleted: Set(false),
            ..Default::default()
        };

         prepare_data.insert(&self.db).await
    }

    pub async fn create_user_verification(
        &self,
        user:&user_credential::Model
    )->Result<user_verification::Model,DbErr>{
        let current_date = chrono::DateTime::<FixedOffset>::default().naive_local();

        let uuid = uuid::Uuid::new_v4();
        let verification = user_verification::ActiveModel{
            id: Set(uuid.to_string()),
            code: Set("1234".to_string()),
            verification_type: Set(VerificationType::Otp),
            user_id: Set(Some(user.id.to_string())),
            created_at: Set(current_date),
            updated_at: Set(current_date),
            deleted: Set(false),
            ..Default::default()
        };

        verification.insert(&self.db).await
    }

    pub async fn get_current_sign_in(
        &self,
        user_id: String,
    ) -> Option<(user_login::Model, Option<user_credential::Model>)> {
        let current_sign_in = user_login::Entity::find()
            .filter(user_login::Column::UserId.eq(user_id))
            .find_also_related(user_credential::Entity)
            .one(&self.db)
            .await;

        let result = match current_sign_in {
            Ok(v) => v,
            Err(_) => None,
        };

        result
    }

    pub async  fn get_session_user(&self,user_id:String)->Result<String, String>{
        let current_user = self.cache.get_connection()
            .unwrap()
            .get_db();

        Ok("".to_string())
    }
}
