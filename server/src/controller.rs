use crate::service::Service;
use crate::structs::{GetLinkById, SaveLink};
use actix_web::web::Json;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Serialize;
use serde_json;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ResponseOkJson<'a> {
    /// Status code
    code: i32,
    /// The code for short link
    data: &'a String,
}

#[derive(Serialize, ToSchema)]
pub struct ResponseErrJson {
    /// Status Code
    code: i32,
    /// Error message
    error: String,
}

/// Get link by id.
///
/// One could call the api endpoint with following curl.
/// ```text
/// curl localhost:8080/asdf
/// ```
#[utoipa::path(
get,
operation_id = "get_long_original_link",
responses(
(status = 200, description = "Link by id", body = ResponseOkJson, examples(
("First" = (summary = "First success", description = "First success", value = json!(ResponseOkJson{code: 200, data: &"https://mysite.com/kekw?one=one&two=two".to_string()}))),
("Second" = (summary = "Second success", description = "Second success", value = json!(ResponseOkJson{code: 200, data: &"https://mysite.com/".to_string()}))),
)),

(status = 400, description = "No such links", body = ResponseErrJson, examples(
("First" = (summary = "First fail", description = "First fail", value = json!(ResponseErrJson{code: 400, error: "No such links!".to_string()}))),
))
),
params(
("id" = String, Path, description = "Short link database id to get original link for redirect", example = "H2dr", description = "Value of short `Link` for the getting original long `Link` to redirect."),
),
)]
#[get("/{link_id}")]
pub async fn get_link_by_id(path: web::Path<GetLinkById>) -> impl Responder {
    let link = Service::get_link_by_short(path.link_id.as_str());
    match link {
        Ok(val) => {
            let res = ResponseOkJson {
                code: 200,
                data: &val.original_link,
            };
            HttpResponse::Ok().json(res)
        }
        Err(err) => {
            let res = ResponseErrJson {
                code: 400,
                error: err.to_string(),
            };
            HttpResponse::BadRequest().json(res)
        }
    }
}

/// Set long link to get short.
///
/// One could call the api endpoint with following curl.
/// ```text
/// curl localhost:8080/set -d '{"link": "https://mysite.com"}'
/// ```
#[utoipa::path(
post,
operation_id = "set_short_link",
responses(
(status = 200, description = "Convert long link to short", body = ResponseOkJson, example = json!(ResponseOkJson{code: 200, data: &"H2dQ".to_string()})),
),
request_body(content = SaveLink, example = json!(SaveLink{link: "https://mysite.com".to_string()})),
)]
#[post("/set")]
pub async fn save_link(req: Json<SaveLink>) -> impl Responder {
    let link = Service::create_link(req.link.as_str());
    let res = ResponseOkJson {
        code: 200,
        data: &link.shared_link,
    };

    HttpResponse::Ok().json(res)
}
