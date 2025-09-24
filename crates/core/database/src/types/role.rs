/// User role in the system
#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "USER_ROLE")]
#[sqlx(rename_all = "lowercase")]
pub enum Role {
    /// Regular user
    Regular,
    /// User with moderator rights
    Moderator,
    /// Administrator
    Admin,
}
