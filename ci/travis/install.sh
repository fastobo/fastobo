#!/bin/sh

set -e

. $(dirname $0)/functions.sh

# --- Setup cargo-tarpaulin ----------------------------------------------------------

LATEST=$(cargo search cargo-tarpaulin | grep cargo-tarpaulin | cut -f2 -d"\"")
log Downloading cargo-tarpaulin v$LATEST
URL="https://github.com/xd009642/tarpaulin/releases/download/${LATEST}/cargo-tarpaulin-${LATEST}-travis.tar.gz"
curl -SsL "$URL" | tar xzvC "$HOME/.cargo/bin"

# --- Setup cargo-cache ------------------------------------------------------

log Installing latest cargo-cache
cargo install -f cargo-cache --no-default-features --features ci-autoclean --root "$HOME/.cargo"
