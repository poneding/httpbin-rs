use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(ci_test))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/ci-test")]
async fn ci_test() -> impl Responder {
    HttpResponse::Ok().body("Update to trigger CI/CD")
}
