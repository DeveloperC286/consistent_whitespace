#!/usr/bin/env sh

set -o errexit
set -o xtrace

if [ "$#" -ne 1 ]; then
	echo "Usage: $0 RELEASE_TAG"
	echo "$#"
	exit 1
fi

RELEASE="$1"

target=$(rustc -vV | grep host | cut -d ' ' -f 2)
tar -czvf "${target}.tar.gz" -C "target/release" "consistent_whitespace"
gh release upload "${RELEASE}" "${target}.tar.gz"
rm "${target}.tar.gz"
