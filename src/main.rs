use actix_web::{App, HttpServer};
use openapi::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
mod openapi;

const VERSION: &str = "0.1.0";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // 配置 OpenAPI 文档
        let mut doc = ApiDoc::openapi();
        doc.info.version = VERSION.to_string();

        App::new()
            // 配置 Swagger UI 服务
            .service(SwaggerUi::new("/openapi/{_:.*}").url("/openapi.json", doc))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
