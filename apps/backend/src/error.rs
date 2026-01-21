use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use snafu::Snafu;

pub type Result<T> = std::result::Result<T, Error>;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Http server host binding failed: {}", external_err))]
    HttpServerHost {
        #[snafu(source)]
        external_err: std::io::Error,
    },
    #[snafu(display("Http server io failed: {}", external_err))]
    HttpServerIO {
        #[snafu(source)]
        external_err: std::io::Error,
    },
}

impl Error {
    pub fn info(&self) -> (StatusCode, ErrorResponse) {
        match self {
            // ===== 400 Series Error =====

            // ===== 500 Series Error =====
            Error::HttpServerHost { external_err } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    tag: "Http server host binding failed".to_string(),
                    message: Some(external_err.to_string()),
                },
            ),
            Error::HttpServerIO { external_err } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    tag: "Http server failed".to_string(),
                    message: Some(external_err.to_string()),
                },
            ),
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let (status_code, err_resp) = self.info();
        if status_code.as_u16() >= 400 {
            tracing::error!("Api Return Error: {err_resp:?}, detail: {self}");
        }
        HttpResponse::build(status_code).json(err_resp)
    }
}

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}
