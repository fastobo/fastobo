#!/bin/sh

. $(dirname $(dirname $0))/functions.sh

# --- Install Python ---------------------------------------------------------

if [ "$PYTHON" = "python3.7" ]; then
  log Updating Python to v${PYTHON#python}
  brew unlink python
  brew install https://raw.githubusercontent.com/Homebrew/homebrew-core/master/Formula/python.rb
else
  log Using Python ${PYTHON#python}
fi


# --- Install Rust -----------------------------------------------------------

log Installing Rust nightly
curl -sSf https://build.travis-ci.org/files/rustup-init.sh | sh -s -- --default-toolchain=nightly -y


# --- Install Python requirements --------------------------------------------

log Installing Python requirements
$PYTHON -m pip install -r "$TRAVIS_BUILD_DIR/ci/requirements.txt"
