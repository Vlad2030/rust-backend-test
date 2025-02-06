use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[derive(sqlx::FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: Option<String>,
    pub name: String,
    pub premium: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Users {
    pub count: i32,
    pub limit: i32,
    pub offset: i32,
    pub users: Vec<User>,
}
