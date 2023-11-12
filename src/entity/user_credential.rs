//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;

use super::sea_orm_active_enums::AuthProvider;
use super::sea_orm_active_enums::Status;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_credential")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
    pub email: String,
    pub full_name: String,
    pub password: String,
    pub status: Status,
    pub auth_provider: AuthProvider,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::admin::Entity")]
    Admin,
    #[sea_orm(has_many = "super::admin_access::Entity")]
    AdminAccess,
    #[sea_orm(has_many = "super::device::Entity")]
    Device,
    #[sea_orm(has_many = "super::user_login::Entity")]
    UserLogin,
    #[sea_orm(has_many = "super::user_profile::Entity")]
    UserProfile,
    #[sea_orm(has_many = "super::user_verification::Entity")]
    UserVerification,
}

impl Related<super::admin::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Admin.def()
    }
}

impl Related<super::admin_access::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AdminAccess.def()
    }
}

impl Related<super::device::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Device.def()
    }
}

impl Related<super::user_login::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserLogin.def()
    }
}

impl Related<super::user_profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserProfile.def()
    }
}

impl Related<super::user_verification::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserVerification.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
