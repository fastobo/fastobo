#!/bin/sh

. $(dirname $(dirname $0))/functions.sh

# --- Test with coverage -----------------------------------------------------

cargo tarpaulin -v -p fastobo --out Xml --ciserver travis-ci
