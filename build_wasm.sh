#!/usr/bin/env bash
set -e
echo "Building OniAnvil WebAssembly (WASM) Release..."
trunk build index.html --release --dist docs/app --public-url ./
echo "OniAnvil WebAssembly build successful in docs/app/"
