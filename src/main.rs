use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};

#[get("/helloworld")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!!!")
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(index)
            .service(healthcheck)
    );

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(init)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
