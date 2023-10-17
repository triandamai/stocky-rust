use crate::models::user::{GetListUsersResponse, UserInfoRequest};
use chrono::Local;
use entity::{user, user_info};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
    QuerySelect, Set,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserRepository {
    pub db_conn: DatabaseConnection,
}

impl UserRepository {
    pub async fn get_users(&self) -> Vec<GetListUsersResponse> {
        let mut result: Vec<GetListUsersResponse> = Vec::new();
        let data = user::Entity::find()
            .find_with_related(entity::user_info::Entity)
            .all(&self.db_conn)
            .await
            .into_iter();
        for item in data {
            result.push(GetListUsersResponse {
                user: item[0].0.to_owned(),
                user_info: item[0].1.to_owned(),
            })
        }
        result
    }

    pub async fn insert_user(&self, data: user::ActiveModel) -> Option<user::Model> {
        let result = data.insert(&self.db_conn).await;
        if result.is_err() {
            return None;
        }
        return Some(result.unwrap());
    }

    pub async fn get_user_by_email(&self, email: &String) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find()
            .filter(user::Column::Email.contains(email))
            .limit(1)
            .one(&self.db_conn)
            .await
    }

    pub async fn get_user_info_by_id(
        &self,
        user_info_id:&String
    )->Result<Vec<user_info::Model>,DbErr>{
     user_info::Entity::find_by_id(user_info_id)
        .all(&self.db_conn)
        .await
    }

    pub async fn insert_user_info(
        &self,
        user_id: &String,
        infos: &Vec<UserInfoRequest>,
    ) -> Result<Vec<user_info::Model>, DbErr> {
        let user = user::Entity::find_by_id(user_id).one(&self.db_conn).await;

        match user {
            Err(err) => Err(err),
            Ok(value) => {
                let user = value.unwrap();

                let mut data_info: Vec<user_info::ActiveModel> = Vec::new();
                let mut save_for_returning: Vec<user_info::Model> = Vec::new();

                for info in infos {
                    let date = Local::now().naive_local();
                    let id = Uuid::new_v4().as_simple().to_string();
                    data_info.push(user_info::ActiveModel {
                        id: Set(id.clone()),
                        user_id: Set(user.id.to_string()),
                        name: Set(info.name.to_string()),
                        value: Set(info.value.to_string()),
                        created_at: Set(Some(date)),
                        updated_at: Set(Some(date)),
                        ..Default::default()
                    });
                    save_for_returning.push(user_info::Model {
                        id,
                        user_id: user.id.to_string(),
                        name: info.name.to_string(),
                        value: info.value.to_string(),
                        created_at: Some(date),
                        updated_at: Some(date)
                    });
                }

        
                    user_info::Entity::insert_many(data_info)
                    .exec(&self.db_conn)
                    .await?;
                

                Ok(save_for_returning.to_owned())
            }
        }
    }

}
