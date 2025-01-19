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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetUsers {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

impl Default for GetUsers {
    fn default() -> Self {
        Self {
            limit: Some(20),
            offset: Some(0),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateUser {
    pub username: Option<String>,
    pub name: Option<String>,
}

impl Default for CreateUser {
    fn default() -> Self {
        Self {
            username: None,
            name: Some("User".into()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeleteUser {
    pub id: Option<uuid::Uuid>,
}

impl Default for DeleteUser {
    fn default() -> Self {
        Self {
            id: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ErrorResponse {
    pub code: u16,
    pub error: String,
    pub message: String,
}
