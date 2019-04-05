#!/bin/sh

log() {
	tput bold
	tput setaf 2
	printf "%12s " $1
	tput sgr0
	shift 1
	echo $@
}

# --- Install python requirements -------------------------------------------
if [ "$TRAVIS_BUILD_STAGE_NAME" != "Rust" ]; then
	log Install Python requirements
	pip install -r "$TRAVIS_BUILD_DIR/ci/requirements.txt"
fi


# --- Setup Rust ------------------------------------------------------------
if [ "$TRAVIS_BUILD_STAGE_NAME" != "Rust" ]; then
	log Installing Rust nightly
	curl -sSf https://build.travis-ci.org/files/rustup-init.sh | sh -s -- --default-toolchain=nightly -y
fi


# --- Setup sscache ---------------------------------------------------------
LATEST=$(cargo search sccache | grep sccache | cut -f2 -d"\"")
LOCAL=$(sccache --version 2>/dev/null | cut -f2 -d" " || echo "none")

if [ "$LATEST" != "$LOCAL" ]; then
	log Downloading sccache v$LATEST
	URL="https://github.com/mozilla/sccache/releases/download/${LATEST}/sccache-${LATEST}-x86_64-unknown-linux-musl.tar.gz"
	curl -SsL $URL | tar xzv -C /tmp
	mv "/tmp/sccache-${LATEST}-x86_64-unknown-linux-musl/sccache" "$HOME/.cargo/bin/sccache"
else
	log Using cached sccache v$LOCAL
fi


# --- Setup cargo-tarpaulin -------------------------------------------------
LATEST=$(cargo search cargo-tarpaulin | grep cargo-tarpaulin | cut -f2 -d"\"")
LOCAL=$(cargo tarpaulin --version 2>/dev/null | cut -d" " -f3 || echo "none")

if [ "$LATEST" != "$LOCAL" ]; then
	log Downloading cargo-tarpaulin v$LATEST
	URL="https://github.com/xd009642/tarpaulin/releases/download/${LATEST}/cargo-tarpaulin-${LATEST}-travis.tar.gz"
	curl -SsL "$URL" | tar xzvC "$HOME/.cargo/bin"
else
	log Using cached cargo-tarpaulin v$LOCAL
fi
