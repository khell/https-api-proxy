#!/bin/bash

export DOCKER_DEFAULT_PLATFORM=linux/amd64
export IMAGE_TAG_BASE=ghcr.io/khell/https-api-proxy

# Build the image on the current commit hash
docker buildx build --platform linux/amd64 --tag $IMAGE_TAG_BASE:$(git rev-parse --short HEAD) .

# Also tag as latest
docker tag $IMAGE_TAG_BASE:$(git rev-parse --short HEAD) $IMAGE_TAG_BASE:latest

# Push to remote
docker push $IMAGE_TAG_BASE:$(git rev-parse --short HEAD)
docker push $IMAGE_TAG_BASE:latest
