#!/bin/sh

log() {
		tput bold
		tput setaf 2
		printf "%12s " $1
		tput sgr0
		shift 1
		echo $@
}


# --- Publish crate to `crates.io` ---------------------------------------------

cargo publish --token $CRATES_IO_TOKEN


# --- Update release tags using Chandler ---------------------------------------

export GEM_PATH="$(ruby -r rubygems -e 'puts Gem.user_dir')"
export PATH="${GEM_PATH}/bin:$PATH"

log "  Installing" "chandler gem"
gem install --user-install chandler

log "    Updating" "GitHub release notes"
chandler push --github="$TRAVIS_REPO_SLUG"
