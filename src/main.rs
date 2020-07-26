use actix_web::{App, HttpServer};
use actix_web::http::header::{ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_ORIGIN, CONNECTION, Date, DATE, SERVER};
use actix_web::middleware::Logger;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(||
        App::new()
            .wrap(Logger::default())
            .wrap(
                actix_web::middleware::DefaultHeaders::new()
                    .header(DATE, Date::now().to_string())
                    .header(SERVER, "actix-web/3.0.0-beta.1")
                    .header(CONNECTION, "keep-alive")
                    .header(ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
                    .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            )
            .configure(httpbin_rs::routes::add_routes)
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
