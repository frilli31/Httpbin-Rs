use actix_web::{HttpResponse, web};

use crate::handlers::return_info_request::return_info_request;

pub fn add_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/get")
                .route(web::get().to(return_info_request))
                .route(web::to(HttpResponse::MethodNotAllowed))
        )
        .service(
            web::resource("/post")
                .route(web::post().to(return_info_request))
                .route(web::to(HttpResponse::MethodNotAllowed))
        )
        .service(
            web::resource("/put")
                .route(web::put().to(return_info_request))
                .route(web::to(HttpResponse::MethodNotAllowed))
        )
        .service(
            web::resource("/delete")
                .route(web::delete().to(return_info_request))
                .route(web::to(HttpResponse::MethodNotAllowed))
        )
        .service(
            web::resource("/patch")
                .route(web::patch().to(return_info_request))
                .route(web::to(HttpResponse::MethodNotAllowed))
        )
    ;
}