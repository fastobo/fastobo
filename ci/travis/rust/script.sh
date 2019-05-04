#!/bin/sh

. $(dirname $(dirname $0))/functions.sh

# --- Test with coverage -----------------------------------------------------

for pkg in 'fastobo' 'fastobo-syntax' 'fastobo2owl'; do
	cargo tarpaulin -v -p $pkg --out Xml --ciserver travis-ci
done

# --- Run examples -----------------------------------------------------------

log Running librarian.rs on ms.obo
cargo script examples/librarian.rs -- tests/data/plana.obo
