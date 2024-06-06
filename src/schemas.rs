use serde;


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub is_premium: bool,
    pub active: bool,
    pub created_at: u32,
    pub updated_at: u32,
}


#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Users {
    pub count: usize,
    pub users: Vec<User>,
}


#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreateUser {
    pub username: String,
    pub name: String,
    pub is_premium: bool,
}


#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreatedUser {
    pub created: bool,
    pub user: User,
}


#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct DeletedUser {
    pub deleted: bool,
    pub user: User,
}


#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct HTTPError {
    pub detail: String,
}