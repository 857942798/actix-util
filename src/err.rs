use super::define::Error as StdError;
use super::define::*;
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde_derive::Serialize;
use serde_json::json;
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::Utf8Error,
};
pub type HttpResult<I> = Result<I, Error>;

#[derive(Debug)]
pub struct Error {
    real_error: Option<ExtraDescError>,
    status: StatusCode,
}

#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    err_type: String,
    desc: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorWrapper {
    status: u16,
    details: Vec<ErrorDetail>,
}

impl ErrorWrapper {
    fn new_from_error(err: &Error) -> ErrorWrapper {
        if let Some(real_error) = &err.real_error {
            let err_detail = ErrorDetail {
                err_type: real_error.err.reason_en().expect("unkown err").to_string(),
                desc: real_error.desc.clone(),
            };
            ErrorWrapper {
                status: err.status.as_u16(),
                details: vec![err_detail],
            }
        } else {
            ErrorWrapper {
                status: err.status.as_u16(),
                details: vec![],
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorOutTpl {
    error: ErrorWrapper,
}

impl ErrorOutTpl {
    fn new_from_error(err: &Error) -> ErrorOutTpl {
        ErrorOutTpl {
            error: ErrorWrapper::new_from_error(&err),
        }
    }
}

impl Error {
    pub fn new(code: StatusCode) -> Self {
        Error {
            real_error: None,
            status: code,
        }
    }

    pub fn err(mut self, e: ExtraDescError) -> Self {
        self.real_error = Some(e);
        self
    }

    pub fn not_find(mut self, msg: &str) -> Self {
        self.real_error = Some(DataBaseNotFound.from_desc(msg));
        self
    }

    pub fn invalid_data(mut self, msg: &str) -> Self {
        self.real_error = Some(InvalidMessageData.from_desc(msg));
        self
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self.real_error)
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        if self.real_error.is_some() {
            HttpResponse::build(status_code).json(json!(ErrorOutTpl::new_from_error(self)))
        } else {
            let std_err = StdError(5001);
            let err_ext = ExtraDescError {
                err: std_err,
                desc: "发生意外错误".to_string(),
            };
            let err = Error {
                status: status_code,
                real_error: Some(err_ext),
            };
            HttpResponse::build(status_code).json(json!(ErrorOutTpl::new_from_error(&err)))
        }
    }

    fn status_code(&self) -> StatusCode {
        self.status
    }
}

impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Self {
        Error::new(StatusCode::INTERNAL_SERVER_ERROR).invalid_data(error.to_string().as_str())
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::new(StatusCode::INTERNAL_SERVER_ERROR).invalid_data(error.to_string().as_str())
    }
}
