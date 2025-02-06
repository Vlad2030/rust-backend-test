use {
    crate::models,
    derive_more::{Display, Error as DError},
};

#[derive(Debug, Display, DError, Clone)]
pub enum Error {
    #[display(fmt = "{} was not found", entity)]
    NotFound {
        entity: String,
    },

    #[display(fmt = "Internal Error {}", err)]
    Internal {
        err: String,
    },

    #[display(fmt = "Database Error {}", error)]
    Database {
        error: String,
    },

    #[display(
        fmt = "Invalid field: `{}`, it should be {}",
        field,
        explanation
    )]
    InvalidField {
        field: String,
        explanation: String,
    },

    #[display(
        fmt = "Username `{}` is busy, try another",
        username
    )]
    BusyUsername {
        username: String,
    },
}

impl Default for Error {
    fn default() -> Self {
        Self::Internal {
            err: "please try again later".into(),
        }
    }
}

impl Error {
    pub fn name(&self) -> String {
        match self {
            Self::NotFound {
                ..
            } => "NotFound".into(),
            Self::Internal {
                ..
            } => "InternalError".into(),
            Self::Database {
                ..
            } => "DatabaseError".into(),
            Self::InvalidField {
                ..
            } => "InvalidField".into(),
            Self::BusyUsername {
                ..
            } => "BusyUsername".into(),
        }
    }
}

impl ntex::web::error::WebResponseError for Error {
    fn status_code(&self) -> ntex::http::StatusCode {
        match *self {
            Self::NotFound {
                ..
            } => ntex::http::StatusCode::NOT_FOUND,
            Self::Internal {
                ..
            } => ntex::http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::Database {
                ..
            } => ntex::http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidField {
                ..
            } => ntex::http::StatusCode::BAD_REQUEST,
            Self::BusyUsername {
                ..
            } => ntex::http::StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(
        &self,
        _: &ntex::web::HttpRequest,
    ) -> ntex::web::HttpResponse {
        let status_code = self.status_code();
        let error_response = models::response::error::Error {
            code: status_code.as_u16(),
            error: self.name(),
            message: self.to_string(),
        };
        ntex::web::HttpResponse::build(status_code)
            .json::<models::response::error::Error>(&error_response)
    }
}

impl From<ntex::web::error::StateExtractorError> for Error {
    fn from(err: ntex::web::error::StateExtractorError) -> Self {
        Error::Internal {
            err: format!("{:?}", err),
        }
    }
}

impl From<ntex::web::error::QueryPayloadError> for Error {
    fn from(err: ntex::web::error::QueryPayloadError) -> Self {
        Error::Internal {
            err: format!("{:?}", err),
        }
    }
}

impl From<ntex::web::error::PathError> for Error {
    fn from(err: ntex::web::error::PathError) -> Self {
        Error::Internal {
            err: format!("{:?}", err),
        }
    }
}
