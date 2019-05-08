#!/bin/sh -e

. $(dirname $(dirname $0))/functions.sh

# --- Test with coverage -----------------------------------------------------

cargo tarpaulin -v -p "fastobo" -p "fastobo-syntax" -p "fastobo2owl" --out Xml --ciserver travis-ci

# --- Run examples -----------------------------------------------------------

log Running librarian.rs on ms.obo
cargo script examples/librarian.rs -- tests/data/plana.obo
