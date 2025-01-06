# 集成 OpenAPI

OpenAPI 是一种用于定义 API 的规范，它可以让开发者更好地理解和使用 API。
在本项目中，我们将集成 [utoipa](https://github.com/juhaku/utoipa) 生成 OpenAPI 文档。

## 添加依赖

```bash
cargo add utoipa
cargo add utoipa-swagger-ui --features=actix-web
```

## 添加 `src/openapi.rs`

```rust
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(title = "httpbin-rs", description = "httpbin implemented in Rust"),
    paths()
)]
pub(crate) struct ApiDoc;
```

> 当然，后续我们添加新的接口时，需要不断的修改该文件，例如在 `#[openapi] paths` 中添加新的路由。
>
> 具体配置可以参考 [utoipa](https://github.com/juhaku/utoipa)。

## `src/main.rs` 中添加 OpenAPI 路由

添加 OpenApi 路由，集成 Swagger UI。

```rust
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
```

## 运行

```bash
cargo run
```

Swagger 地址： <http://localhost:8080/openapi/>

API 地址： <http://localhost:8080/openapi.json>
