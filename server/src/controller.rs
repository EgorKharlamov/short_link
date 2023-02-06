use crate::service::Service;
use crate::structs::{GetLinkById, SaveLink};
use actix_web::web::Json;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct ResponseOkJson<'a> {
    code: i32,
    data: &'a String,
}

#[derive(Serialize)]
struct ResponseErrJson {
    code: i32,
    error: String,
}

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

#[post("/set")]
pub async fn save_link(req: Json<SaveLink>) -> impl Responder {
    let link = Service::create_link(req.link.as_str());
    let res = ResponseOkJson {
        code: 200,
        data: &link.shared_link,
    };

    HttpResponse::Ok().json(res)
}
