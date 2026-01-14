#!/bin/bash
set -e

# To use this script, place a text file with repository name in parent directory
REPO=$(cat ../repository.txt)
VERSION=`cargo metadata --format-version=1 --no-deps | jq -r '.packages[0].version'`
NAME=`cargo metadata --format-version=1 --no-deps | jq -r '.packages[0].name'`

docker buildx build -f Dockerfile \
  -t "$REPO/$NAME:$VERSION" \
  -t "$REPO/$NAME:latest" \
  --build-arg TARGET_PATH=target/x86_64-unknown-linux-musl/release/ \
  .

echo "Built $REPO/$NAME:$VERSION"