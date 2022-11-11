#!/bin/bash

paths=$(find . -path "./*/Cargo.toml")
for path in $paths; do
  echo "${path}"
  cargo test --manifest-path "${path}"
done
