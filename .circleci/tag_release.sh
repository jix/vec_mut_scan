#!/bin/bash
set -eu

VER=$(sed -ne 's/^version = "\(.*\)".*/\1/;T;p;q' Cargo.toml)

if ! git tag -l v$VER | grep -q .; then
    git tag -a v$VER -m "Release of version $VER"
fi

git describe --tags --match='v[0-9]*' --dirty='-d' '--always' | sed 's/^v//'
