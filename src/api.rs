use serde_json;
use ntex::web;

use crate::schemas;
use crate::storage;


#[web::get("/users")]
async fn get_users() -> Result<web::HttpResponse, web::Error> {
    unsafe {
        let users: &Vec<schemas::User> = &storage::USERS;
        let count: usize = users.len();

        let json_data: serde_json::Value = serde_json::to_value(
            schemas::Users{count, users: users.to_vec()}
        )?;

        return Ok(web::HttpResponse::Ok()
                    .content_type("application/json")
                    .json(&json_data))
    }
}


#[web::get("/users/{id}")]
async fn get_user_by_id(path: web::types::Path<(i32,)>) -> Result<web::HttpResponse, web::Error> {
    unsafe {
        let user_id: i32 = path.into_inner().0;

        let database_users: &Vec<schemas::User> = &storage::USERS;

        let mut _user: Option<&schemas::User> = None;

        for database_user in database_users {
            if user_id == database_user.id {
                _user = Some(database_user);
                break;
            }
        }

        if _user.is_none() == true {
            let error_json = serde_json::to_string(
                &schemas::HTTPError{
                    detail: format!("user {} not found", user_id),
            })?;

            return Err(web::error::ErrorBadRequest(error_json).into());
        }

        let json_data: serde_json::Value = serde_json::to_value(_user)?;

        return Ok(web::HttpResponse::Ok()
                    .content_type("application/json")
                    .json(&json_data))
    }
}


#[web::post("/users")]
async fn create_user(user: web::types::Json<schemas::CreateUser>) -> Result<web::HttpResponse, web::Error> {
    unsafe {
        let user: schemas::CreateUser = user.into_inner();

        let mut _user: Option<&schemas::User> = None;

        let datetime: u32 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
                            .expect("")
                            .as_secs()
                            as u32;

        let count: usize = storage::USERS.len();

        let user: schemas::User = schemas::User{
            id: (count as i32) + 1,
            username: user.username,
            name: user.name,
            is_premium: user.is_premium,
            active: true,
            created_at: datetime,
            updated_at: datetime,
        };

        storage::USERS.push(user.clone());

        let json_data: serde_json::Value = serde_json::to_value(
            schemas::CreatedUser{
                created: true,
                user,
        })?;

        return Ok(web::HttpResponse::Ok()
                    .content_type("application/json")
                    .json(&json_data))
    }
}


#[web::delete("/users/{id}")]
async fn delete_user(path: web::types::Path<(i32,)>) -> Result<web::HttpResponse, web::Error> {
    unsafe {
        let user_id: i32 = path.into_inner().0;

        let database_users: &Vec<schemas::User> = &storage::USERS;

        let mut _user: Option<&schemas::User> = None;

        for count in 0..=database_users.len() {
            if user_id == database_users[count].id {
                let user_id: usize = count;
                let user: &schemas::User = &database_users[user_id];
                let _user: Option<schemas::User> = Some(user.clone());
                break;
            }
        }

        if _user.is_none() == true {
            let error_json = serde_json::to_string(
                &schemas::HTTPError{
                    detail: format!("user {} not found", user_id),
            })?;

            return Err(web::error::ErrorBadRequest(error_json).into());
        }

        storage::USERS.remove(user_id.try_into().unwrap());

        let json_data: serde_json::Value = serde_json::to_value(
            schemas::DeletedUser{
                deleted: true,
                user: _user.unwrap().clone(),
        })?;

        return Ok(web::HttpResponse::Ok()
                    .content_type("application/json")
                    .json(&json_data))
    }
}