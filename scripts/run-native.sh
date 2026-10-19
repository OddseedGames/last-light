#!/bin/sh
set -eu
. "$(dirname -- "$0")/rust-env.sh"
exec cargo run --release --locked
