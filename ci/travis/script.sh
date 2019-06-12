#!/bin/sh -e

. $(dirname $0)/functions.sh

# --- Test with coverage -----------------------------------------------------

log Checking code without features
cargo check --release --no-default-features

log Checking code with feature \`semantics\`
cargo check --release --no-default-features --features semantics

log Checking code with feature \`semantics\`
cargo check --release --no-default-features --features display

log Checking code with feature \`semantics\`, \`display\`
cargo check --release --no-default-features --features display,semantics

log Checking code with all features
cargo check --release --all-features

log Measuring code coverage with Tarpaulin
cargo tarpaulin --release -v --out Xml --ciserver travis-ci

# --- Run examples -----------------------------------------------------------

log Running librarian.rs on ms.obo
cargo run --release --example librarian -- tests/data/plana.obo
