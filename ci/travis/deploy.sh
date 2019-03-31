#!/bin/sh

log() {
		tput bold
		tput setaf 2
		printf "%12s " $1
		tput sgr0
		shift 1
		echo $@
}

error() {
		tput bold
		tput setaf 1
		printf "%12s " $1
		tput sgr0
		shift 1
		echo $@
}



# --- Publish crate to `crates.io` ---------------------------------------------

case "$TRAVIS_TAG" in
	v*-syntax)
			cd "$TRAVIS_BUILD_DIR/fastobo-syntax"
			log Publishing fastobo-syntax ${TRAVIS_TAG%-syntax}
			cargo publish --token $CRATES_IO_TOKEN
			;;
	v*)
			cd "$TRAVIS_BUILD_DIR"
			log Publishing fastobo $TRAVIS_TAG
			cargo publish --token $CRATES_IO_TOKEN
			;;
	*)
			error Error invalid or missing tag: $TRAVIS_TAG
			exit 1
			;;
esac


# --- Update release tags using Chandler ---------------------------------------

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
										--git="../.git" \
										--tag-prefix=syntax/v
			;;
	v*)
			cd "$TRAVIS_BUILD_DIR"
			chandler push --github="$TRAVIS_REPO_SLUG"
			;;
	*)
			error Error invalid or missing tag: $TRAVIS_TAG
			exit 1
			;;
esac
