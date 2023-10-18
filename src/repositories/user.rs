use crate::{
    common::utils::create_uuid,
    models::user::{GetListUsersResponse, UserInfoRequest},
};
use chrono::Local;
use entity::{user, user_info};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
    QuerySelect, Set,
};

#[derive(Debug, Clone)]
pub struct UserRepository {
    pub db_conn: DatabaseConnection,
}

impl UserRepository {
    pub async fn get_users(&self) -> Vec<GetListUsersResponse> {
        let result = user::Entity::find()
            .find_with_related(entity::user_info::Entity)
            .all(&self.db_conn)
            .await;

        if result.is_err() {
            return Vec::new();
        }

        result
            .unwrap()
            .into_iter()
            .map(|val| GetListUsersResponse {
                user: val.0.to_owned(),
                user_info: val.1.to_owned(),
            })
            .collect()
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

    pub async fn register_user(

    ){
        
    }

    pub async fn get_user_info_by_id(
        &self,
        user_info_id: &String,
    ) -> Result<Vec<user_info::Model>, DbErr> {
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

                let data: Vec<user_info::ActiveModel> = infos
                    .into_iter()
                    .map(|info| {
                        let date = Local::now().naive_local();
                        let id = create_uuid();
                        return user_info::ActiveModel {
                            id: Set(id.clone()),
                            user_id: Set(user.id.to_string()),
                            name: Set(info.name.to_string()),
                            value: Set(info.value.to_string()),
                            created_at: Set(Some(date)),
                            updated_at: Set(Some(date)),
                            ..Default::default()
                        };
                    })
                    .collect();

                let saved: Vec<user_info::Model> = data
                    .clone()
                    .into_iter()
                    .map(|info| {
                        return user_info::Model {
                            id: info.id.into_value().unwrap().to_string(),
                            user_id: user.id.to_string(),
                            name: info.name.into_value().unwrap().to_string(),
                            value: info.value.into_value().unwrap().to_string(),
                            created_at: info.created_at.unwrap(),
                            updated_at: info.updated_at.unwrap(),
                        };
                    })
                    .collect();

                user_info::Entity::insert_many(data)
                    .exec(&self.db_conn)
                    .await?;

                Ok(saved.to_owned())
            }
        }
    }
}
