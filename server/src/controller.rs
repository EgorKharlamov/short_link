use crate::service::Service;
use crate::structs::{GetLinkById, SaveLink};
use actix_web::web::Json;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ResponseOkJson<'a> {
    code: i32,
    data: &'a String,
}

#[derive(Serialize, ToSchema)]
pub struct ResponseErrJson {
    code: i32,
    error: String,
}

/// Get link by id.
///
/// One could call the api endpoint with following curl.
/// ```text
/// curl localhost:8080/asdf
/// ```
#[utoipa::path(
responses(
(status = 200, description = "Link by id", body = [ResponseOkJson]),
(status = 400, description = "No such links", body = [ResponseErrJson])
)
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
responses(
(status = 200, description = "Convert long link to short", body = [ResponseOkJson]),
)
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
