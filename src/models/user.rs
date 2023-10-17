use serde::{Deserialize,Serialize};
use entity::{user,user_info};

#[derive(Serialize,Deserialize)]
pub struct UserInfoRequest{
    pub name:String,
    pub value:String
}

#[derive(Serialize,Deserialize)]
pub struct AddUserInfoRequest{
    pub user_id:String,
    pub user_info:Vec<UserInfoRequest>
}

#[derive(Serialize,Deserialize)]
pub struct GetListUsersResponse{
    pub user:user::Model,
    pub user_info:Vec<user_info::Model>
}