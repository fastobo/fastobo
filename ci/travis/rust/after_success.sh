#!/bin/sh -e

. $(dirname $(dirname $0))/functions.sh


# --- Publish crate to `crates.io` -------------------------------------------

case "$TRAVIS_TAG" in
	# Release fastobo-syntax
	v*-syntax)
		cd "$TRAVIS_BUILD_DIR/fastobo-syntax"
		log Publishing fastobo-syntax ${TRAVIS_TAG%-syntax}
		cargo publish --token $CRATES_IO_TOKEN
		;;
	# Release fastobo
	v*)
		cd "$TRAVIS_BUILD_DIR/fastobo"
		log Publishing fastobo $TRAVIS_TAG
		cargo publish --token $CRATES_IO_TOKEN
		;;
esac


# --- Update release tags using Chandler -------------------------------------

export GEM_PATH="$(ruby -r rubygems -e 'puts Gem.user_dir')"
export PATH="${GEM_PATH}/bin:$PATH"

log Installing chandler gem
gem install --user-install chandler

log Updating GitHub release notes
case "$TRAVIS_TAG" in
	v*-syntax)
		cd "$TRAVIS_BUILD_DIR/fastobo-syntax"
		chandler push --github="$TRAVIS_REPO_SLUG" \
			--changelog="CHANGELOG.md" \
			--git="../.git"
			;;
  v*-py)
    cd "$TRAVIS_BUILD_DIR/fastobo-py"
    chandler push --github="$TRAVIS_REPO_SLUG" \
      --changelog="CHANGELOG.md" \
      --git="../.git"
      ;;
	v*)
		cd "$TRAVIS_BUILD_DIR/fastobo"
		chandler push --github="$TRAVIS_REPO_SLUG" \
			--changelog="CHANGELOG.md" \
			--git="../.git"
			;;
esac
