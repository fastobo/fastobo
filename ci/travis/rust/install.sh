#!/bin/sh

. $(dirname $(dirname $0))/functions.sh


# --- Setup sccache ----------------------------------------------------------

LATEST=$(cargo search sccache | grep sccache | cut -f2 -d"\"")
LOCAL=$(sccache --version 2>/dev/null | cut -f2 -d" " || echo "none")

if [ "$LATEST" != "$LOCAL" ]; then
	log Downloading sccache v$LATEST
  URL="https://github.com/mozilla/sccache/releases/download/${LATEST}/sccache-${LATEST}-x86_64-unknown-linux-musl.tar.gz"
	curl -SsL $URL | tar xzv -C /tmp
	mkdir -p "$HOME/.cargo/bin"
	mv "/tmp/sccache-${LATEST}-x86_64-unknown-linux-musl/sccache" "$HOME/.cargo/bin/sccache"
	chmod +x "$HOME/.cargo/bin/sccache"
else
	log Using cached sccache v$LOCAL
fi


# --- Setup cargo-tarpaulin --------------------------------------------------

LATEST=$(cargo search cargo-tarpaulin | grep cargo-tarpaulin | cut -f2 -d"\"")
LOCAL=$(cargo tarpaulin --version 2>/dev/null | cut -d" " -f3 || echo "none")

if [ "$LATEST" != "$LOCAL" ]; then
  log Downloading cargo-tarpaulin v$LATEST
  URL="https://github.com/xd009642/tarpaulin/releases/download/${LATEST}/cargo-tarpaulin-${LATEST}-travis.tar.gz"
  curl -SsL "$URL" | tar xzvC "$HOME/.cargo/bin"
else
  log Using cached cargo-tarpaulin v$LOCAL
fi
