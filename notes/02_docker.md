# 支持 Docker 部署

## .dockerignore

创建 `.dockerignore` 文件，在构建 Docker 镜像时排除不必要的文件，内容如下：

```txt
.git/
.gitignore

target/

notes/
README.md
```

## Dockerfile

项目根目录下创建 `Dockerfile` 文件，内容如下：

```dockerfile
FROM rust:alpine AS builder
WORKDIR /app

# 安装 musl-dev，用于编译静态链接的二进制
RUN apk add --no-cache musl-dev
COPY . .
RUN cargo build --release

FROM alpine AS runtime
COPY --from=builder /app/target/release/httpbin-rs /httpbin-rs
ENTRYPOINT [ "/httpbin-rs" ]
```

但是，这种方式每次构建都会重新编译整个项目，构建效率较低。

为了提高构建的效率，目前一般采用以下两种方式：

1. 第一种方式：

```dockerfile
# 编译层
FROM rust:alpine AS builder
WORKDIR /app

# 安装 musl-dev，用于编译静态链接的二进制
RUN apk add --no-cache musl-dev

COPY Cargo.toml Cargo.lock ./
# 创建虚拟的 src 目录(用于提前缓存依赖)
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm src/main.rs

# 拷贝实际代码
COPY . .
# 更新 src/main.rs 的时间戳，触发重新编译
RUN touch src/main.rs
RUN cargo build --release

# 运行层
FROM alpine
COPY --from=builder /app/target/release/httpbin-rs /httpbin-rs
ENTRYPOINT ["/httpbin-rs"]
```

2. 第二种方式：

参考：[cargo-chef](https://github.com/LukeMathWalker/cargo-chef)

```dockerfile
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
```

> 注意：在 `runtime` 阶段，我们没有使用 `alpine` 镜像，因为 `alpine` 镜像的 libc 是 `musl`，而 `debian` 镜像的 libc 是 `glibc`，这样可以避免一些潜在的问题。

目前更多推荐的是第二种方式。

## 构建

创建编译脚本文件 `build.sh`，内容如下：

```bash
#!/bin/bash

IMAGE=httpbin-rs
VERSION=v$(cargo read-manifest| jq -r .version)

if ! docker buildx inspect rs-builder &> /dev/null; then
  echo "🦀 - Creating builder: rs-builder"
  docker buildx create --name rs-builder &> /dev/null
fi

echo "🦀 - Using builder: rs-builder"
docker buildx use rs-builder

echo "🦀 - Building docker image..."

docker buildx build . --platform linux/amd64,linux/arm64 \
  -f Dockerfile \
  -t poneding/$IMAGE:$VERSION \
  -t registry.cn-hangzhou.aliyuncs.com/pding/$IMAGE:$VERSION \
  --push

echo "🦀 - Done!"
```

运行构建脚本：

```bash
chmod +x build.sh
./build.sh
```

## 测试

运行 Docker 镜像：

```bash
docker run -d -p 8080:8080 poneding/httpbin-rs:v$(cargo read-manifest| jq -r .version)
```

浏览器访问 [`http://localhost:8080`](http://localhost:8080)，如果看到 `Hello world!`，则说明镜像可用。
