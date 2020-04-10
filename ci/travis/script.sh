#!/bin/sh -e

. $(dirname $0)/functions.sh

# --- Test without features --------------------------------------------------

log Testing without default features
cargo tarpaulin --release -v --out Xml --ciserver travis-ci --no-default-features

# --- Test with coverage -----------------------------------------------------

log Testing with default features
cargo tarpaulin --release -v --out Xml --ciserver travis-ci

# --- Run examples -----------------------------------------------------------

log Running librarian.rs on ms.obo
cargo run --example librarian -- tests/data/plana.obo
