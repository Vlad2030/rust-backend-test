use {crate::core, crate::utils};

#[ntex::web::get("/health")]
async fn healthcheck() -> core::result::ApiResult {
    Ok(utils::ok_response_json(
        &serde_json::json!({"successful": true}),
    ))
}
