use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::prelude::*;

use crate::extension::postgres::Type;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_type(
                Type::create()
                    .as_enum(Status::Table)
                    .values(Status::iter().skip(1))
                    .to_owned()
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(AuthProvider::Table)
                    .values(AuthProvider::iter().skip(1))
                    .to_owned()
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(VerificationType::Table)
                    .values(VerificationType::iter().skip(1))
                    .to_owned()
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(PostType::Table)
                    .values(PostType::iter().skip(1)).to_owned()
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(CommentType::Table)
                    .values(CommentType::iter().skip(1)).to_owned()
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(AttachmentType::Table)
                    .values(AttachmentType::iter().skip(1)).to_owned()
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserCredential::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserCredential::Id)
                            .string()
                            .primary_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserCredential::Email)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserCredential::FullName).string().not_null())
                    .col(ColumnDef::new(UserCredential::Password).string().not_null())
                    .col(
                        ColumnDef::new(UserCredential::Status)
                            .enumeration(Status::Table, Status::iter().skip(1))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserCredential::AuthProvider)
                            .enumeration(AuthProvider::Table, AuthProvider::iter().skip(1))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserCredential::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UserCredential::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(UserCredential::Deleted).boolean().not_null().default(false))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserProfile::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserProfile::Id).string().primary_key().not_null())
                    .col(ColumnDef::new(UserProfile::Key).string().not_null())
                    .col(ColumnDef::new(UserProfile::Value).string().not_null())
                    .col(ColumnDef::new(UserProfile::UserId).string())
                    .col(
                        ColumnDef::new(UserProfile::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UserProfile::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(UserProfile::Deleted).boolean().not_null().default(false))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserVerification::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserVerification::Id)
                            .string()
                            .primary_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserVerification::Code).string().not_null())
                    .col(
                        ColumnDef::new(UserVerification::VerificationType)
                            .enumeration(VerificationType::Table, VerificationType::iter().skip(1))
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserVerification::UserId).string())
                    .col(
                        ColumnDef::new(UserVerification::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UserVerification::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(UserVerification::Deleted).boolean().not_null().default(false))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserReport::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserReport::Id)
                            .string()
                            .primary_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserReport::Reason).string().not_null())
                    .col(ColumnDef::new(UserReport::UserId).string())
                    .col(ColumnDef::new(UserReport::ReportedBy).string())
                    .col(ColumnDef::new(UserReport::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(UserReport::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(UserReport::Deleted).boolean().not_null().default(false))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Admin::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Admin::Id).string().primary_key().not_null())
                    .col(ColumnDef::new(Admin::UserId).string())
                    .col(ColumnDef::new(Admin::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Admin::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Admin::Deleted).boolean().not_null().default(false))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SystemAccess::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SystemAccess::Id)
                            .string()
                            .primary_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(SystemAccess::Name).string().not_null())
                    .col(ColumnDef::new(SystemAccess::Permission).string().not_null())
                    .col(ColumnDef::new(SystemAccess::Group).string().not_null())
                    .col(
                        ColumnDef::new(SystemAccess::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(SystemAccess::UpdatedAt)
                            .timestamp()
                            .not_null().default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(SystemAccess::Deleted).boolean().not_null().default(false))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AdminAccess::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AdminAccess::Id)
                            .string()
                            .primary_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(AdminAccess::AdminId).string())
                    .col(ColumnDef::new(AdminAccess::AccessId).string())
                    .col(
                        ColumnDef::new(AdminAccess::CreatedAt)
                            .timestamp()
                            .not_null().default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(AdminAccess::UpdatedAt)
                            .timestamp()
                            .not_null().default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(AdminAccess::Deleted).boolean().not_null().default(false))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Device::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Device::Id).string().primary_key().not_null())
                    .col(ColumnDef::new(Device::DeviceName).string())
                    .col(ColumnDef::new(Device::DeviceId).string())
                    .col(ColumnDef::new(Device::DeviceOs).string())
                    .col(ColumnDef::new(Device::UserId).string())
                    .col(ColumnDef::new(Device::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Device::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Device::Deleted).boolean().not_null().default(false))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserLogin::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserLogin::Id)
                            .string()
                            .primary_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserLogin::Ip).string())
                    .col(ColumnDef::new(UserLogin::Token).string())
                    .col(ColumnDef::new(UserLogin::LoginAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(UserLogin::UserId).string())
                    .col(ColumnDef::new(UserLogin::DeviceId).string())
                    .col(ColumnDef::new(UserLogin::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(UserLogin::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(UserLogin::Deleted).boolean().not_null().default(false))
                    .to_owned(),
            )
            .await?;

        manager.create_table(
            Table::create()
                .table(Hashtag::Table)
                .if_not_exists()
                .col(ColumnDef::new(Hashtag::Table).string())
                .col(ColumnDef::new(Hashtag::Id).string().not_null().primary_key())
                .col(ColumnDef::new(Hashtag::Value).string())
                .col(ColumnDef::new(Hashtag::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                .col(ColumnDef::new(Hashtag::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                .col(ColumnDef::new(Hashtag::Deleted).boolean().not_null().default(false))
                .to_owned()
        ).await?;

        manager.create_table(
            Table::create()
                .table(Post::Table)
                .if_not_exists()
                .col(ColumnDef::new(Post::Id).string().primary_key().not_null())
                .col(ColumnDef::new(Post::CreatedBy).string())
                .col(ColumnDef::new(Post::PostId).string())
                .col(ColumnDef::new(Post::Body).text())
                .col(ColumnDef::new(Post::LikesCount).big_integer().not_null())
                .col(ColumnDef::new(Post::CommentsCount).big_integer().not_null())
                .col(
                    ColumnDef::new(Post::PostType)
                        .enumeration(PostType::Table, PostType::iter().skip(1))
                        .not_null()
                )
                .col(ColumnDef::new(Post::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                .col(ColumnDef::new(Post::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                .col(ColumnDef::new(Post::Deleted).boolean().not_null().default(false))
                .to_owned()
        ).await?;

        manager
            .create_table(
                Table::create()
                    .table(PostAttachment::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(PostAttachment::Id).string()
                        .primary_key()
                        .not_null())
                    .col(ColumnDef::new(PostAttachment::PostId).string())
                    .col(ColumnDef::new(PostAttachment::AttachmentType)
                        .enumeration(AttachmentType::Table,AttachmentType::iter().skip(1)))
                    .col(ColumnDef::new(PostAttachment::MimeType).string().not_null())
                    .col(ColumnDef::new(PostAttachment::Ext).string().not_null())
                    .col(ColumnDef::new(PostAttachment::Value).string())
                    .col(ColumnDef::new(PostAttachment::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(PostAttachment::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(PostAttachment::Deleted).boolean().not_null().default(false))
                    .to_owned()
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(PostComment::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(PostComment::Id).string().primary_key())
                    .col(ColumnDef::new(PostComment::Body).string())
                    .col(ColumnDef::new(PostComment::UserId).string())
                    .col(ColumnDef::new(PostComment::ReplyTo).string())
                    .col(ColumnDef::new(PostComment::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(PostComment::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(PostComment::Deleted).boolean().not_null().default(false))
                    .to_owned()
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PostMention::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(PostMention::Table).string().primary_key())
                    .col(ColumnDef::new(PostMention::PostId).string())
                    .col(ColumnDef::new(PostMention::UserId).string())
                    .col(ColumnDef::new(PostMention::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(PostMention::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(PostMention::Deleted).boolean().not_null().default(false))
                    .to_owned()
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PostHashtag::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(PostHashtag::Id).string().primary_key())
                    .col(ColumnDef::new(PostHashtag::PostId).string())
                    .col(ColumnDef::new(PostHashtag::HashtagId).string())
                    .col(ColumnDef::new(PostHashtag::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(PostHashtag::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(PostHashtag::Deleted).boolean().not_null().default(false))
                    .to_owned()
            )
            .await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk-user-profile")
                .from(UserProfile::Table, UserProfile::UserId)
                .to(UserCredential::Table, UserCredential::Id)
                .to_owned()
        ).await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk-user-verification")
                .from(UserVerification::Table, UserVerification::UserId)
                .to(UserCredential::Table, UserCredential::Id)
                .to_owned()
        ).await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk-user-report")
                .from(UserReport::Table, UserReport::UserId)
                .to(UserCredential::Table, UserCredential::Id)
                .to_owned()
        ).await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk-user-report-by")
                .from(UserReport::Table, UserReport::ReportedBy)
                .to(UserCredential::Table, UserCredential::Id)
                .to_owned()
        ).await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk-user-admin")
                .from(Admin::Table, Admin::UserId)
                .to(UserCredential::Table, UserCredential::Id)
                .to_owned()
        ).await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk-admin-user")
                .from(AdminAccess::Table, AdminAccess::AdminId)
                .to(UserCredential::Table, UserCredential::Id)
                .to_owned()
        ).await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk-admin-access")
                .from(AdminAccess::Table, AdminAccess::AccessId)
                .to(SystemAccess::Table, SystemAccess::Id)
                .to_owned()
        ).await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk-user-device")
                .from(Device::Table, Device::UserId)
                .to(UserCredential::Table, UserCredential::Id)
                .to_owned()
        ).await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk-user-login")
                .from(UserLogin::Table, UserLogin::UserId)
                .to(UserCredential::Table, UserCredential::Id)
                .to_owned()
        ).await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk-user-login-device")
                .from(UserLogin::Table, UserLogin::DeviceId)
                .to(Device::Table, Device::Id)
                .to_owned()
        ).await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk-post-created-by")
                .from(Post::Table, Post::CreatedBy)
                .to(UserCredential::Table, UserCredential::Id)
                .to_owned()
        ).await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk-post-attachment")
                .from(PostAttachment::Table, PostAttachment::PostId)
                .to(Post::Table, Post::Id)
                .to_owned()
        ).await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk-post-repost-from")
                .from(Post::Table, Post::PostId)
                .to(Post::Table, Post::Id)
                .to_owned()
        ).await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk-comment-reply-to")
                .from(PostComment::Table, PostComment::ReplyTo)
                .to(UserCredential::Table, UserCredential::Id)
                .to_owned()
        ).await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk-mention-to")
                .from(PostMention::Table, PostMention::UserId)
                .to(UserCredential::Table, UserCredential::Id)
                .to_owned()
        ).await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk-hashtag-post")
                .from(PostHashtag::Table, PostHashtag::PostId)
                .to(Post::Table, Post::Id)
                .to_owned()
        ).await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk-hashtag-id")
                .from(PostHashtag::Table, PostHashtag::HashtagId)
                .to(Hashtag::Table, Hashtag::Id)
                .to_owned()
        ).await?;

        Ok(())
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();
    }
}

#[derive(DeriveIden)]
enum UserCredential {
    Table,
    Id,
    Email,
    Password,
    AuthProvider,
    Status,
    FullName,
    CreatedAt,
    UpdatedAt,
    Deleted,
}

#[derive(Iden, EnumIter)]
pub enum Status {
    Table,
    #[iden = "ACTIVE"]
    ACTIVE,
    #[iden = "INACTIVE"]
    INACTIVE,
    #[iden = "SUSPENDED"]
    SUSPENDED,
    #[iden = "WAITING_CONFIRMATION"]
    WAITING,
}

#[derive(Iden, EnumIter)]
pub enum AuthProvider {
    Table,
    #[iden = "GOOGLE"]
    GOOGLE,
    #[iden = "BASIC"]
    BASIC,
    #[iden = "FACEBOOK"]
    FACEBOOK,
    #[iden = "APPLE"]
    APPLE,
    #[iden = "GITHUB"]
    GITHUB,
    #[iden = "MICROSOFT"]
    MICROSOFT,
    #[iden = "TWITTER"]
    TWITTER,
}

#[derive(DeriveIden)]
enum UserVerification {
    Table,
    Id,
    Code,
    UserId,
    VerificationType,
    CreatedAt,
    UpdatedAt,
    Deleted,
}

#[derive(Iden, EnumIter)]
pub enum VerificationType {
    Table,
    #[iden = "OTP"]
    OTP,
    #[iden = "RESET"]
    RESET,
    #[iden = "ACTIVATION"]
    ACTIVATION,
}

#[derive(DeriveIden)]
enum UserProfile {
    Table,
    Id,
    Key,
    Value,
    UserId,
    CreatedAt,
    UpdatedAt,
    Deleted,
}

#[derive(DeriveIden)]
enum UserReport {
    Table,
    Id,
    UserId,
    ReportedBy,
    Reason,
    CreatedAt,
    UpdatedAt,
    Deleted,
}

#[derive(DeriveIden)]
enum Admin {
    Table,
    Id,
    UserId,
    CreatedAt,
    UpdatedAt,
    Deleted,
}

#[derive(DeriveIden)]
enum AdminAccess {
    Table,
    Id,
    AdminId,
    AccessId,
    CreatedAt,
    UpdatedAt,
    Deleted,
}

#[derive(DeriveIden)]
enum SystemAccess {
    Table,
    Id,
    Name,
    Group,
    Permission,
    CreatedAt,
    UpdatedAt,
    Deleted,
}

#[derive(DeriveIden)]
enum UserLogin {
    Table,
    Id,
    UserId,
    DeviceId,
    Ip,
    Token,
    LoginAt,
    CreatedAt,
    UpdatedAt,
    Deleted,
}

#[derive(DeriveIden)]
enum Device {
    Table,
    Id,
    UserId,
    DeviceName,
    DeviceId,
    DeviceOs,
    CreatedAt,
    UpdatedAt,
    Deleted,
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    CreatedBy,
    PostId,
    Location,
    Body,
    Post,
    PostType,
    LikesCount,
    CommentsCount,
    CreatedAt,
    UpdatedAt,
    Deleted,
}

#[derive(DeriveIden)]
enum PostMention {
    Table,
    Id,
    UserId,
    PostId,
    CreatedAt,
    UpdatedAt,
    Deleted,
}

#[derive(DeriveIden)]
enum PostHashtag {
    Table,
    Id,
    PostId,
    HashtagId,
    CreatedAt,
    UpdatedAt,
    Deleted,
}

#[derive(DeriveIden)]
enum Hashtag {
    Table,
    Id,
    Value,
    CreatedAt,
    UpdatedAt,
    Deleted,
}

#[derive(Iden, EnumIter)]
pub enum PostType {
    Table,
    #[iden = "BASIC"]
    BASIC,
    #[iden = "POLLING"]
    POLLING,
}

#[derive(DeriveIden)]
enum PostAttachment {
    Table,
    Id,
    PostId,
    AttachmentType,
    MimeType,
    Ext,
    Value,
    CreatedAt,
    UpdatedAt,
    Deleted,
}

#[derive(Iden, EnumIter)]
pub enum AttachmentType {
    Table,
    #[iden = "IMAGE"]
    IMAGE,
    #[iden = "VIDEO"]
    VIDEO,
}

#[derive(DeriveIden)]
enum Mention {
    Table,
    Id,
    UserId,
    CreatedAt,
    UpdatedAt,
    Deleted,
}

#[derive(DeriveIden)]
enum PostComment {
    Table,
    Id,
    Body,
    ReplyTo,
    UserId,
    CreatedAt,
    UpdatedAt,
    Deleted,
}

#[derive(DeriveIden)]
enum PostCommentMention {
    Table,
    Id,
    UserId,
    CreatedAt,
    UpdatedAt,
    Deleted,
}

#[derive(Iden, EnumIter)]
pub enum CommentType {
    Table,
    #[iden = "REPLY"]
    REPLY,
    #[iden = "COMMENT"]
    COMMENT,
}

#[derive(DeriveIden)]
enum PostLike {
    Table,
    Id,
    UserId,
    PostId,
    CreatedAt,
    UpdatedAt,
    Deleted,
}
