# 初始化项目

```bash
cargo init httpbin-rs && cd httpbin-rs

# 添加对 actix-web 的依赖
cargo add actix-web
```

实现一个简单的 HTTP 服务：

```rust
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
```

运行：

```bash
cargo run

# 打开另一个终端
curl http://localhost:8080
```
