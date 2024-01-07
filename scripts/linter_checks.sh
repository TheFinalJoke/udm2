#!/bin/bash

set -eu pipefail

# PROTOLINT

echo "Running 'protolint lint protos/'"

/usr/local/bin/protolint lint protos/

echo "Running Cargo Clippy"
cargo clippy

echo "Running Cargo Fmt"

cargo fmt --all

echo "Running ShellCheck"
shellcheck scripts/*.sh