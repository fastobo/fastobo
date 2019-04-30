#!/bin/sh

. $(dirname $(dirname $0))/functions.sh

# --- Test with coverage -----------------------------------------------------

cargo tarpaulin -v -p fastobo-syntax --out Xml --ciserver travis-ci
cargo tarpaulin -v -p fastobo --out Xml --ciserver travis-ci

# --- Run examples -----------------------------------------------------------

log Running librarian.rs on ms.obo
cargo script examples/librarian.rs -- tests/data/ms.obo
