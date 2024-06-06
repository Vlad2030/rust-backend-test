use serde;
use serde_json;
use ntex::web;


static mut USERS: Vec<User> = Vec::new();


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct User {
    id: i32,
    username: String,
    name: String,
    is_premium: bool,
    active: bool,
    created_at: u32,
    updated_at: u32,
}


#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Users {
    count: usize,
    users: Vec<User>,
}


#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct CreateUser {
    username: String,
    name: String,
    is_premium: bool,
}


#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct CreatedUser {
    created: bool,
    user: User,
}


#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct DeletedUser {
    deleted: bool,
    user: User,
}


#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct HTTPError {
    detail: String,
}


#[web::get("/users")]
async fn get_users() -> Result<web::HttpResponse, web::Error> {
    unsafe {
        let users: &Vec<User> = &USERS;
        let count: usize = users.len();

        let json_data: serde_json::Value = serde_json::to_value(
            Users{count, users: users.to_vec()}
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

        let database_users: &Vec<User> = &USERS;

        let mut _user: Option<&User> = None;

        for database_user in database_users {
            if user_id == database_user.id {
                _user = Some(database_user);
                break;
            }
        }

        if _user.is_none() == true {
            let error_json = serde_json::to_string(
                &HTTPError{
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
async fn create_user(user: web::types::Json<CreateUser>) -> Result<web::HttpResponse, web::Error> {
    unsafe {
        let user: CreateUser = user.into_inner();

        let mut _user: Option<&User> = None;

        let datetime: u32 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
                            .expect("Time went backwards")
                            .as_secs()
                            as u32;

        let count: usize = USERS.len();

        let user: User = User{
            id: (count as i32) + 1,
            username: user.username,
            name: user.name,
            is_premium: user.is_premium,
            active: true,
            created_at: datetime,
            updated_at: datetime,
        };

        USERS.push(user.clone());

        let json_data: serde_json::Value = serde_json::to_value(
            CreatedUser{
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

        let database_users: &Vec<User> = &USERS;

        let mut _user: Option<&User> = None;

        for count in 0..=database_users.len() {
            if user_id == database_users[count].id {
                let user_id: usize = count;
                let user: &User = &database_users[user_id];
                let _user: Option<User> = Some(user.clone());
                break;
            }
        }

        if _user.is_none() == true {
            let error_json = serde_json::to_string(
                &HTTPError{
                    detail: format!("user {} not found", user_id),
            })?;

            return Err(web::error::ErrorBadRequest(error_json).into());
        }

        USERS.remove(user_id.try_into().unwrap());

        let json_data: serde_json::Value = serde_json::to_value(
            DeletedUser{
                deleted: true,
                user: _user.unwrap().clone(),
        })?;

        return Ok(web::HttpResponse::Ok()
                    .content_type("application/json")
                    .json(&json_data))
    }
}


#[ntex::main]
async fn main() -> std::io::Result<()> {
    let cpu_count: usize = std::thread::available_parallelism()?.get();

    let host: &str = "0.0.0.0";
    let port = 8000;

    println!("Service started! http://{}:{}/", host, port);

    web::HttpServer::new(|| {
        web::App::new()
        .service((
            get_users,
            get_user_by_id,
            create_user,
            delete_user,
        ))
        .wrap(web::middleware::Logger::default())
    })
    .bind((host, port))?
    .workers(cpu_count)
    .run()
    .await
}
