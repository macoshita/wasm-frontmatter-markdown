#!/usr/bin/env bash
wasm-pack build --scope macoshita -t nodejs
cargo run --example gen_highlight_css
cargo run --example modify_package_json
