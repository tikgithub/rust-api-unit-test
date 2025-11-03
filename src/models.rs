#[derive(Debug, Deserialize, Serialize)]
pub enum UserRole {
    Admin,
    Moderator,
    User,
}
