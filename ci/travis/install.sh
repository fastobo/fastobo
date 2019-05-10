#!/bin/sh -e

. $(dirname $0)/functions.sh

# --- Setup cargo-tarpaulin --------------------------------------------------

LATEST=$(cargo search cargo-tarpaulin | grep cargo-tarpaulin | cut -f2 -d"\"")
LOCAL=$(cargo tarpaulin --version 2>/dev/null | cut -d" " -f3 || echo "none")

if [ "$LATEST" != "$LOCAL" ]; then
	log Downloading cargo-tarpaulin v$LATEST
	URL="https://github.com/xd009642/tarpaulin/releases/download/${LATEST}/cargo-tarpaulin-${LATEST}-travis.tar.gz"
	curl -SsL "$URL" | tar xzC "$HOME/.cargo/bin"
else
	log Using cached cargo-tarpaulin v$LOCAL
fi
