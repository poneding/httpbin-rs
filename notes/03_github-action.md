# 集成 GitHub Action 构建

## 1. 创建 GitHub Action 配置文件

在项目根目录下创建 `.github/workflows` 目录，并在该目录下创建 `docker-build.yml` 文件，内容如下：

```yaml
name: docker-build

env:
  IMAGE: httpbin-rs
  TAG: latest
on:
  push:
    paths:
      - "src/**"
      - "Cargo.toml"
      - "Cargo.lock"
  workflow_dispatch:

jobs:
  build:
    runs-on: ubuntu-latest
    environment: default # 利用 Environment(default) 中配置的 Secrets 和 Variables
    steps:
      - uses: actions/checkout@v4

      - name: Set up QEMU
        uses: docker/setup-qemu-action@v3

      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3

      - name: Login to Docker Hub
        uses: docker/login-action@v3
        with:
          username: ${{ vars.DOCKERHUB_USERNAME }}
          password: ${{ secrets.DOCKERHUB_TOKEN }}

      - name: Login to Aliyun ACR
        uses: docker/login-action@v3
        with:
          registry: ${{ vars.ALIYUNACR_REGISTRY }}
          username: ${{ vars.ALIYUNACR_USERNAME }}
          password: ${{ secrets.ALIYUNACR_TOKEN }}

      - name: Build and push
        uses: docker/build-push-action@v6
        with:
          platforms: linux/amd64,linux/arm64
          context: .
          file: Dockerfile
          push: true
          tags: ${{ vars.DOCKERHUB_USERNAME }}/${{ env.IMAGE }}:${{ env.TAG }},${{ vars.ALIYUNACR_REGISTRY }}/${{ vars.ALIYUNACR_NAMESPACE }}/${{ env.IMAGE }}:${{ env.TAG }}
```

当目录下的 `src` 目录、`Cargo.toml` 或 `Cargo.lock` 文件发生变化时，触发构建；也可以手动触发构建。

## 2. 配置 GitHub Secrets

在项目的 `Settings` -> `Environments` -> `New environment` 中创建一个环境。

1. 配置 Environment secrets：

- `DOCKERHUB_TOKEN`：Docker Hub Token
- `ALIYUNACR_TOKEN`：阿里云 ACR Token

2. 配置 Environment variables：

- `DOCKERHUB_USERNAME`：Docker Hub 用户
- `ALIYUNACR_REGISTRY`：阿里云 ACR Registry
- `ALIYUNACR_USERNAME`：阿里云 ACR 用户名
- `ALIYUNACR_NAMESPACE`：阿里云 ACR 命名空间

## 3. 提交代码

提交代码，之后每次代码提交，符合条件时会触发构建。

可能本次提交，由于 src 目录以及 Cargo.toml 或 Cargo.lock 文件未发生变化，不会触发构建。此时我们可以手动触发构建。
