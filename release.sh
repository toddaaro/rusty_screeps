#!/bin/bash

# first argument is a version string, second argument is a message

cargo fmt
sed -i "3 s/version.*/version = \"${1}\"/" Cargo.toml
git add src
git commit -am "${2}"
git tag -a "v${1}" -m "${2}"
git push
git push origin "v${1}"
