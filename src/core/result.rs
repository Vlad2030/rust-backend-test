use crate::core;

pub type Result<T> = std::result::Result<T, core::error::Error>;
pub type ApiResult = Result<ntex::web::HttpResponse>;
pub type QueryResult<T> =
    std::result::Result<ntex::web::types::Query<T>, ntex::web::error::QueryPayloadError>;
