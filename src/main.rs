use ntex::web;

pub mod schemas;
pub mod storage;
pub mod api;


static BACKEND_HOST: &str = "0.0.0.0";
static BACKEND_PORT: i32 = 8000;


#[ntex::main]
async fn main() -> std::io::Result<()> {
    let cpu_count: usize = std::thread::available_parallelism()?.get();

    println!("Service started! http://{}:{}/", BACKEND_HOST, BACKEND_PORT);

    web::HttpServer::new(|| {
        web::App::new()
        .service((
            api::get_users,
            api::get_user_by_id,
            api::create_user,
            api::delete_user,
        ))
        .wrap(web::middleware::Logger::default())
    })
    .bind((BACKEND_HOST, BACKEND_PORT as u16))?
    .workers(cpu_count)
    .run()
    .await
}
