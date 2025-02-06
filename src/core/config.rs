use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct App {
    pub title: String,
    pub version: String,
    pub host: String,
    pub port: u32,
    #[allow(dead_code)]
    pub log_level: log::Level,
    pub workers: usize,
    pub database: Database,
}

impl App {
    pub fn new() -> Self {
        Self {
            title: std::env::var("BACKEND_TITLE").unwrap_or_default(),
            version: std::env::var("BACKEND_VERSION").unwrap_or_default(),
            host: std::env::var("BACKEND_HOST").unwrap_or_default(),
            port: std::env::var("BACKEND_PORT")
                .unwrap()
                .parse::<u32>()
                .unwrap_or_default(),
            log_level: log::Level::from_str(
                std::env::var("BACKEND_LOG_LEVEL")
                    .unwrap_or("info".into())
                    .as_str(),
            )
            .unwrap_or(log::Level::Info),
            workers: std::env::var("BACKEND_WORKERS")
                .unwrap()
                .parse::<usize>()
                .unwrap_or_default(),
            database: Database::new(),
        }
    }

    pub fn bind(&self) -> (String, u16) {
        (self.clone().host, self.clone().port as u16)
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            title: "rust-backend".into(),
            version: "0.3.0".into(),
            host: "localhost".into(),
            port: 6969,
            log_level: log::Level::Info,
            workers: 1_usize,
            database: Database::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Database {
    database: String,
    host: String,
    port: u32,
    user: String,
    password: String,
}

impl Database {
    pub fn new() -> Self {
        Self {
            database: std::env::var("DB_NAME").unwrap_or_default(),
            host: std::env::var("DB_CONTAINER_HOST").unwrap_or_default(),
            port: std::env::var("DB_CONTAINER_PORT")
                .unwrap()
                .parse::<u32>()
                .unwrap_or_default(),
            user: std::env::var("DB_USER").unwrap_or_default(),
            password: std::env::var("DB_PASSWORD").unwrap_or_default(),
        }
    }

    pub fn url(self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database,
        )
    }
}

impl Default for Database {
    fn default() -> Self {
        Self {
            database: "db".into(),
            host: "localhost".into(),
            port: 5432,
            user: "user".into(),
            password: "password".into(),
        }
    }
}
