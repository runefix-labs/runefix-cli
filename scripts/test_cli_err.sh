#!/usr/bin/env bash
set -e

# Slice: Invalid Inputs (Expected Errors)

cargo run --quiet -- slice " [0:3] " "Hello, World!"      # ❌ Err!
cargo run --quiet -- slice "(0:3)" "Hello, World!"        # ❌ Err!
cargo run --quiet -- slice "[0 : 3]" "Hello, World!"      # ❌ Err!
cargo run --quiet -- slice "[-3]" "Hello, World!"         # ❌ Err!
cargo run --quiet -- slice "[1:2:1]" "Hello, World!"      # ❌ Err!
cargo run --quiet -- slice "[-3:3]" "Hello, World!"       # ❌ Err!
cargo run --quiet -- slice "[1:-1]" "Hello, World!"       # ❌ Err!
cargo run --quiet -- slice "[0:14]" "Hello, World!"       # ❌ Err!
