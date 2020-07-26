use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::common::request_info::RequestInfo;

pub async fn return_info_request(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(RequestInfo::new(req))
}