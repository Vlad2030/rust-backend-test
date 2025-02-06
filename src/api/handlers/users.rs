use crate::{core, models, utils};

#[ntex::web::get("/users")]
async fn get_users(
    query: core::result::QueryResult<models::request::user::Get>,
    db_pool: core::database::DbPoolState,
) -> core::result::ApiResult {
    if let Err(e) = query {
        return Err(e.into());
    }

    let users_query = query.unwrap().into_inner();

    let query_limit = users_query.limit.unwrap_or(20_i32); // workaround
    let query_offset = users_query.offset.unwrap_or_default();

    if !(0..=100).contains(&query_limit) {
        return Err(core::error::Error::InvalidField {
            field: "limit".into(),
            explanation: "0 < limit <= 100".into(),
        });
    }

    if query_offset < 0 {
        return Err(core::error::Error::InvalidField {
            field: "offset".into(),
            explanation: "0 < offset".into(),
        });
    }

    let raw_sql_query = "
        SELECT
            id,
            username,
            name,
            premium,
            created_at,
            updated_at
        FROM public.users
        WHERE
            ($1 IS NULL OR id = $1)
            AND ($2 IS NULL OR username = $2)
        OFFSET $3
        LIMIT $4
        ;
    ";

    let users = sqlx::query_as::<_, models::response::user::User>(raw_sql_query)
        .bind::<Option<uuid::Uuid>>(users_query.id)
        .bind::<Option<String>>(users_query.username)
        .bind::<i32>(query_offset)
        .bind::<i32>(query_limit)
        .fetch_all(db_pool.get_ref())
        .await
        .map_err(|err| core::error::Error::Database {
            error: format!("{:?}", err),
        })?;

    let response = utils::response::ok_json(&models::response::user::Users {
        count: users.len() as i32,
        limit: query_limit,
        offset: query_offset,
        users,
    });

    Ok(response)
}

#[ntex::web::post("/users")]
async fn create_user(
    query: core::result::QueryResult<models::request::user::Create>,
    db_pool: core::database::DbPoolState,
) -> core::result::ApiResult {
    if let Err(e) = query {
        return Err(e.into());
    }

    let create_user_query = query.unwrap().into_inner();

    let raw_sql_query = "
        INSERT INTO public.users (
            id,
            username,
            name,
            premium,
            created_at,
            updated_at
        )
        VALUES (
            gen_random_uuid(),
            $1,
            COALESCE($2, 'User'),
            false,
            CURRENT_TIMESTAMP,
            CURRENT_TIMESTAMP
        )
        ON CONFLICT (username) DO NOTHING
        RETURNING
            id,
            username,
            name,
            premium,
            created_at,
            updated_at
        ;
    ";

    let mut transaction = db_pool
        .begin()
        .await
        .map_err(|err| core::error::Error::Database {
            error: format!("{:?}", err),
        })?;

    let user = sqlx::query_as::<_, models::response::user::User>(raw_sql_query)
        .bind::<&Option<String>>(&create_user_query.username)
        .bind::<Option<String>>(create_user_query.name)
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
        Some(u) => utils::response::created_json(&u),
        None => {
            return Err(core::error::Error::BusyUsername {
                username: create_user_query.username.unwrap(),
            })
        },
    };

    Ok(response)
}

#[ntex::web::delete("/users")]
async fn delete_user(
    query: core::result::QueryResult<models::request::user::Delete>,
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

    let raw_sql_query = "
        DELETE FROM public.users
        WHERE
            id = $1
        RETURNING
            id,
            username,
            name,
            premium,
            created_at,
            updated_at
        ;
    ";

    let mut transaction = db_pool
        .begin()
        .await
        .map_err(|err| core::error::Error::Database {
            error: format!("{:?}", err),
        })?;

    let user = sqlx::query_as::<_, models::response::user::User>(raw_sql_query)
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
        Some(u) => utils::response::ok_json(&u),
        None => {
            return Err(core::error::Error::NotFound {
                entity: "User".into(),
            })
        },
    };

    Ok(response)
}

#[ntex::web::put("/users")]
async fn update_user(
    query: core::result::QueryResult<models::request::user::Update>,
    db_pool: core::database::DbPoolState,
) -> core::result::ApiResult {
    if let Err(e) = query {
        return Err(e.into());
    }

    let update_user_query = query.unwrap().into_inner();

    if update_user_query.id.is_none() {
        return Err(core::error::Error::InvalidField {
            field: "id".into(),
            explanation: "not none".into(),
        });
    }

    if let Some(username) = &update_user_query.username {
        let username_exists = sqlx::query_scalar(
            "
                SELECT EXISTS
                    (
                        SELECT 1
                        FROM
                            public.users
                        WHERE
                            username = $1
                            AND id != $2
                    )
                ;
            ",
        )
        .bind::<&String>(username)
        .bind::<Option<uuid::Uuid>>(update_user_query.id)
        .fetch_one(db_pool.get_ref())
        .await
        .map_err(|err| core::error::Error::Database {
            error: format!("{:?}", err),
        })?;

        if username_exists {
            return Err(core::error::Error::BusyUsername {
                username: username.clone(),
            });
        }
    }

    let raw_sql_query = "
        UPDATE public.users
        SET
            username = COALESCE($2, username),
            name = COALESCE($3, name),
            updated_at = CURRENT_TIMESTAMP
        WHERE
            id = $1
            AND (
                $2 IS NULL OR NOT EXISTS (
                    SELECT 1
                    FROM public.users
                    WHERE username = $2
                )
            )
        RETURNING
            id,
            username,
            name,
            premium,
            created_at,
            updated_at
        ;
    ";

    let mut transaction = db_pool
        .begin()
        .await
        .map_err(|err| core::error::Error::Database {
            error: format!("{:?}", err),
        })?;

    let user = sqlx::query_as::<_, models::response::user::User>(raw_sql_query)
        .bind::<uuid::Uuid>(update_user_query.id.unwrap())
        .bind::<Option<String>>(update_user_query.username)
        .bind::<Option<String>>(update_user_query.name)
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
        Some(u) => utils::response::ok_json(&u),
        None => {
            return Err(core::error::Error::NotFound {
                entity: "User".into(),
            })
        },
    };

    Ok(response)
}
