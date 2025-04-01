#!/bin/bash

IMAGE=httpbin-rs
TAG=$(shell cargo read-manifest| jq -r .version)-actix-web

if ! docker buildx inspect rs-builder &> /dev/null; then
  echo "🦀 - Creating builder: rs-builder"
  docker buildx create --name rs-builder &> /dev/null
fi

echo "🦀 - Using builder: rs-builder"
docker buildx use rs-builder

echo "🦀 - Building docker image..."

docker buildx build . --platform linux/amd64,linux/arm64 
  -f Dockerfile \
  -t poneding/$IMAGE:$TAG \
  -t registry.cn-hangzhou.aliyuncs.com/pding/$IMAGE:$TAG \
  --push

echo "🦀 - Done!"