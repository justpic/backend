/// Item visibility
#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "VISIBILITY")]
#[sqlx(rename_all = "lowercase")]
pub enum Visibility {
    /// `Public`:
    /// can be seen by **all users**
    Public,
    /// `Private`:
    /// only the **owner** can see
    Private,
}
