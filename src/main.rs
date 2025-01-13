use actix_web::{
    dev,
    http::{header, StatusCode},
    middleware::{DefaultHeaders, ErrorHandlerResponse, ErrorHandlers, Logger},
    App, HttpServer, Result,
};
use env_logger::Env;
use openapi::ApiDoc;
use std::net::Ipv4Addr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
mod anything;
mod auth;
mod dynamic_data;
mod http_methods;
mod openapi;

const VERSION: &str = "0.1.0";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info")); // 从环境变量中初始化日志，如果没有设置则默认为 info

    HttpServer::new(|| {
        // 配置 OpenAPI 文档
        let mut doc = ApiDoc::openapi();
        doc.info.version = VERSION.to_string();

        App::new()
            .wrap(Logger::default()) // 配置日志中间件
            .wrap(DefaultHeaders::new().add(("X-Version", VERSION))) // 配置默认响应头中间件
            .wrap(ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR, add_error_header))
            // 配置 Swagger UI 服务
            .service(SwaggerUi::new("/openapi/{_:.*}").url("/openapi.json", doc))
            .configure(http_methods::api)
            .configure(auth::api)
            .configure(anything::api)
            .configure(dynamic_data::api)
    })
    // 两种绑定方式
    // .bind("0.0.0.0:8080")? // 同 .bind(("0.0.0.0", 8080))?
    // .bind("[::1]:8080")? // 同 .bind(("::1", 8080))?
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    // .bind((Ipv6Addr::UNSPECIFIED, 8080))?
    .run()
    .await
}

// 为错误响应添加自定义响应头
fn add_error_header<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("ERROR"),
    );

    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}
