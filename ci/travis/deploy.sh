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
	v*)
			log Publishing fastobo $TRAVIS_TAG
			cargo publish --token $CRATES_IO_TOKEN --dry-run
			;;
	syntax/v*)
			cd fastobo-syntax
			log Publishing fastobo-syntax ${TRAVIS_TAG#syntax/}
			cargo publish --token $CRATES_IO_TOKEN --dry-run
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
chandler push --github="$TRAVIS_REPO_SLUG"
