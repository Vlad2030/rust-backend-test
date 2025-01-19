use crate::{core, models, utils};

#[ntex::web::get("/users")]
async fn get_users(
    query: core::result::QueryResult<models::GetUsers>,
    db_pool: core::database::DbPoolState,
) -> core::result::ApiResult {
    if let Err(e) = query {
        return Err(e.into());
    }

    let users_query = query.unwrap().into_inner();
    let query_limit = users_query.limit.unwrap_or_default();
    let query_offset = users_query.offset.unwrap_or_default();

    if query_limit <= 0 || query_limit > 100 {
        return Err(core::error::Error::InvalidField {
            field: "limit".into(),
            explanation: format!("0 <= limit({}) <= 100", query_limit),
        });
    }

    if query_offset < 0 {
        return Err(core::error::Error::InvalidField {
            field: "offset".into(),
            explanation: "0 <= offset".into(),
        });
    }

    let users = sqlx::query_as::<_, models::User>(
        "
        SELECT * FROM users
        OFFSET $1
        LIMIT $2;
        ",
    )
    .bind::<i32>(query_offset.clone())
    .bind::<i32>(query_limit.clone())
    .fetch_all(db_pool.get_ref())
    .await
    .map_err(|err| core::error::Error::Database {
        error: format!("{:?}", err),
    })?;

    let response = utils::ok_response_json(&models::Users {
        count: users.len() as i32,
        limit: query_limit,
        offset: query_offset,
        users,
    });

    Ok(response)
}

#[ntex::web::get("/users/id/{id}")]
async fn get_user_by_id(
    path: core::result::PathResult<uuid::Uuid>,
    db_pool: core::database::DbPoolState,
) -> core::result::ApiResult {
    if let Err(e) = path {
        return Err(e.into());
    }

    let user_id = path.unwrap().into_inner();

    let user = sqlx::query_as::<_, models::User>(
        "
        SELECT * FROM users
        WHERE id = $1;
        ",
    )
    .bind::<uuid::Uuid>(user_id.clone())
    .fetch_optional(db_pool.get_ref())
    .await
    .map_err(|err| core::error::Error::Database {
        error: format!("{:?}", err),
    })?;

    let response = match user {
        Some(u) => utils::ok_response_json(&u),
        None => {
            return Err(core::error::Error::NotFound {
                entity: "User".into(),
            })
        },
    };

    Ok(response)
}

#[ntex::web::get("/users/username/{username}")]
async fn get_user_by_username(
    path: core::result::PathResult<String>,
    db_pool: core::database::DbPoolState,
) -> core::result::ApiResult {
    if let Err(e) = path {
        return Err(e.into());
    }

    let user_username = path.unwrap().into_inner();

    let user = sqlx::query_as::<_, models::User>(
        "
        SELECT * FROM users
        WHERE username = $1;
        ",
    )
    .bind::<String>(user_username.clone())
    .fetch_optional(db_pool.get_ref())
    .await
    .map_err(|err| core::error::Error::Database {
        error: format!("{:?}", err),
    })?;

    let response = match user {
        Some(u) => utils::ok_response_json(&u),
        None => {
            return Err(core::error::Error::NotFound {
                entity: "User".into(),
            })
        },
    };

    Ok(response)
}

#[ntex::web::post("/users")]
async fn create_user(
    query: core::result::QueryResult<models::CreateUser>,
    db_pool: core::database::DbPoolState,
) -> core::result::ApiResult {
    if let Err(e) = query {
        return Err(e.into());
    }

    let create_user_query = query.unwrap().into_inner();

    if create_user_query.username.is_none() {
        return Err(core::error::Error::InvalidField {
            field: "username".into(),
            explanation: "not none".into(),
        });
    }

    let query_username = create_user_query.username.unwrap_or_default();
    let query_name = create_user_query.name.unwrap_or_default();

    let mut transaction = db_pool
        .begin()
        .await
        .map_err(|err| core::error::Error::Database {
            error: format!("{:?}", err),
        })?;

    let user = sqlx::query_as::<_, models::User>(
        "
        INSERT INTO users (
            id,
            username,
            name,
            premium,
            created_at,
            updated_at
        )
        VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6
        )
        ON CONFLICT (username) DO NOTHING
        RETURNING *;
        ",
    )
    .bind::<uuid::Uuid>(uuid::Uuid::new_v4())
    .bind::<String>(query_username.clone())
    .bind::<String>(query_name.clone())
    .bind::<bool>(false)
    .bind::<chrono::DateTime<chrono::Utc>>(chrono::offset::Utc::now())
    .bind::<chrono::DateTime<chrono::Utc>>(chrono::offset::Utc::now())
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|err| core::error::Error::Database {
        error: format!("{:?}", err),
    })?;

    transaction
        .commit()
        .await
        .map_err(|err| core::error::Error::Database {
            error: format!("{:?}", err),
        })?;

    let response = match user {
        Some(u) => utils::created_response_json(&u),
        None => {
            return Err(core::error::Error::BusyUsername {
                username: query_username,
            })
        },
    };

    Ok(response)
}


#[ntex::web::delete("/users")]
async fn delete_user(
    query: core::result::QueryResult<models::DeleteUser>,
    db_pool: core::database::DbPoolState,
) -> core::result::ApiResult {
    if let Err(e) = query {
        return Err(e.into());
    }

    let delete_user_query = query.unwrap().into_inner();

    if delete_user_query.id.is_none() {
        return Err(core::error::Error::InvalidField {
            field: "id".into(),
            explanation: "not none".into(),
        });
    }

    let query_id = delete_user_query.id.unwrap_or_default();

    let mut transaction = db_pool
        .begin()
        .await
        .map_err(|err| core::error::Error::Database {
            error: format!("{:?}", err),
        })?;

    let user = sqlx::query_as::<_, models::User>(
        "
        DELETE FROM users
        WHERE id = $1
        RETURNING *;
        ",
    )
    .bind::<uuid::Uuid>(query_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|err| core::error::Error::Database {
        error: format!("{:?}", err),
    })?;

    transaction
        .commit()
        .await
        .map_err(|err| core::error::Error::Database {
            error: format!("{:?}", err),
        })?;

    let response = match user {
        Some(u) => utils::ok_response_json(&u),
        None => {
            return Err(core::error::Error::NotFound {
                entity: "User".into(),
            })
        },
    };

    Ok(response)
}
