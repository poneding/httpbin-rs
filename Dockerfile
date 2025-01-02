FROM rust AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# 构建依赖，这是一个缓存层
RUN cargo chef cook --release --recipe-path recipe.json
# 构建实际项目
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
COPY --from=builder /app/target/release/httpbin-rs /httpbin-rs
ENTRYPOINT ["/httpbin-rs"]