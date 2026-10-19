#!/usr/bin/env bash
set -euo pipefail
cargo run --locked -q -p cf-cli --bin context-ferry -- example
cargo run --locked -q -p cf-cli --bin context-ferry -- evaluate
