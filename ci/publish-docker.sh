#!/usr/bin/env sh

set -o errexit
set -o xtrace

if [ $# -ne 2 ]; then
	echo "Usage: $0 <release> <target>"
	exit 1
fi

RELEASE="$1"
TARGET="$2"
REPOSITORY="$(basename "$(git rev-parse --show-toplevel)")"
IMAGE="ghcr.io/developerc286/${REPOSITORY}"

docker build --build-arg TARGET="${TARGET}" --tag "${IMAGE}:${RELEASE}" --file Dockerfile .
docker push "${IMAGE}:${RELEASE}"

if [ "${RELEASE#v}" != "${RELEASE}" ]; then
	docker tag "${IMAGE}:${RELEASE}" "${IMAGE}:${RELEASE#v}"
	docker push "${IMAGE}:${RELEASE#v}"
fi
