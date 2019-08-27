#!/bin/sh -e

. $(dirname $0)/functions.sh

# --- Deploy `fastobo-derive-internal` crate ---------------------------------

log Deploying \`fastobo-derive-internal\` v$TRAVIS_TAG
cargo publish --manifest-path derive/Cargo.toml --token $CRATES_IO_TOKEN

sleep 10
cargo update

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
