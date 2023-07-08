pub mod define;
pub mod err;
pub mod query;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use actix_web::http::header;
use actix_web::web::JsonConfig;
use actix_web::{error::InternalError, HttpResponse};
use serde_json::json;

pub fn get_default_jsonconfig() -> JsonConfig {
    JsonConfig::default()
        .limit(1024 * 1024 * 1000)
        .error_handler(|err, _| {
            let err_msg = format!("{:?}", err);
            InternalError::from_response(
                err,
                HttpResponse::BadRequest()
                    .insert_header((header::CONTENT_TYPE, "application/json"))
                    .body(
                        json!({
                            "error":{
                                "status": 500,
                                "details":{
                                    "status_text": err_msg,
                                    "desc": "json解析错误"
                                }
                            }
                        })
                        .to_string(),
                    ),
            )
            .into()
        })
}
