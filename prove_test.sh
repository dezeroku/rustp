#!/usr/bin/env bash
cargo build --release
find prove_examples/ -name "*.rs" | xargs -I _ target/release/rustp _
