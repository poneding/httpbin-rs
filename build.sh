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