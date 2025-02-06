use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Get {
    pub id: Option<uuid::Uuid>,
    pub username: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

impl Default for Get {
    fn default() -> Self {
        Self {
            id: None,
            username: None,
            limit: Some(20_i32),
            offset: Some(0_i32),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Create {
    pub username: Option<String>,
    pub name: Option<String>,
}

impl Default for Create {
    fn default() -> Self {
        Self {
            username: None,
            name: Some("User".into()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Delete {
    pub id: Option<uuid::Uuid>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Update {
    pub id: Option<uuid::Uuid>,
    pub username: Option<String>,
    pub name: Option<String>,
}
