#!/bin/sh
set -eu
. "$(dirname -- "$0")/rust-env.sh"
cargo build --release --target wasm32-unknown-unknown --locked --offline
mkdir -p dist
cp web/index.html web/style.css web/mq_js_bundle.js web/THIRD-PARTY-LICENSES.txt dist/
cp assets/OFL.txt dist/
cp target/wasm32-unknown-unknown/release/last-light.wasm dist/
python3 scripts/package.py
python3 scripts/verify-package.py
