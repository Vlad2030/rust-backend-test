pub fn ok_json<T>(data: &T) -> ntex::web::HttpResponse
where
    for<'de> T: serde::Serialize + serde::Deserialize<'de>,
{
    ntex::web::HttpResponse::Ok().json(data)
}

pub fn created_json<T>(data: &T) -> ntex::web::HttpResponse
where
    for<'de> T: serde::Serialize + serde::Deserialize<'de>,
{
    ntex::web::HttpResponse::Created().json(data)
}
