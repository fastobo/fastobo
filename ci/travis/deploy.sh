#!/bin/sh -e

. $(dirname $0)/functions.sh

# --- Set the version of both crates using the Travis tag --------------------

sed -i 's/version = ".*"/version = "$TRAVIS_TAG"/g' Cargo.toml derive/Cargo.toml

# --- Deploy `fastobo-derive` crate ------------------------------------------

log Deploying \`fastobo-derive\` v$TRAVIS_TAG
cargo publish --manifest-path derive/Cargo.toml --token $CRATES_IO_TOKEN

# --- Deploy `fastobo` crate -------------------------------------------------

log Deploying \`fastobo\` v$TRAVIS_TAG
cargo publish --manifest-path Cargo.toml --token $CRATES_IO_TOKEN


# --- Update GitHub release notes --------------------------------------------

export GEM_PATH="$(ruby -r rubygems -e 'puts Gem.user_dir')"
export PATH="${GEM_PATH}/bin:$PATH"

log Installing chandler gem
gem install --user-install chandler

log Updating GitHub release notes
chandler push --github="$TRAVIS_REPO_SLUG" --changelog="CHANGELOG.md"
