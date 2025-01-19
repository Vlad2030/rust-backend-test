mod api;
mod core;
mod models;
mod utils;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env("BACKEND_LOG_LEVEL");

    let app = core::config::App::new();

    log::info!(
        "Trying connect to database ({})",
        app.clone().database.url()
    );

    let db_pool = core::database::create_pool(app.clone().database.url().as_str()).await;

    if db_pool.is_err() {
        log::error!(
            "Connection failed ({})",
            app.clone().database.url(),
        );
        std::process::exit(1_i32)
    }

    log::info!("{} {}", app.title, app.version);
    log::info!(
        "Service starting.. http://{}:{}",
        app.host,
        app.port,
    );
    ntex::web::HttpServer::new(move || {
        ntex::web::App::new()
            .state(db_pool.clone().unwrap())
            .service((
                api::health::healthcheck,
                api::users::get_users,
                api::users::get_user_by_id,
                api::users::get_user_by_username,
                api::users::create_user,
                api::users::delete_user,
            ))
            .wrap(ntex::web::middleware::Logger::default())
    })
    .bind(app.bind())?
    .workers(app.workers)
    .run()
    .await
}
