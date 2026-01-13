# 初始化项目

## 初始化项目

```bash
cargo init httpbin-rs && cd httpbin-rs

# 添加对 axum 的依赖
cargo add axum
```

## 简单实现

```rust
use axum::{Router, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello world!" }));
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

## 运行

```bash
cargo run
```

访问 [`http://localhost:8080`](http://localhost:8080)，可以看到 `Hello world!`。
