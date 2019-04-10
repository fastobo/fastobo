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



# --- Publish crate to `crates.io` and/or `pypi.org` -------------------------

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
	# Release fastobo-py
	v*-py)
		python setup.py sdist bdist_wheel
		twine upload --skip-existing dist/*.whl dist/*.tar.gz
		;;
	# Release dev version of `fastobo-py`
	*)
		VERSION=$(python setup.py --version)-dev$(git rev-list --count --all)
		sed -i "s/version = $(python setup.py --version)/version = $VERSION/g" setup.cfg
		log Building fastobo-py sdist
		python setup.py sdist		
		log Building fastobo-py wheels
		docker run --rm -v $TRAVIS_BUILD_DIR:/io quay.io/pypa/manylinux1_x86_64 /io/ci/build-wheels.sh
		log Publishing fastobo-py $VERSION
		twine upload --skip-existing dist/*.whl dist/*.tar.gz
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
	v*)
		cd "$TRAVIS_BUILD_DIR/fastobo"
		chandler push --github="$TRAVIS_REPO_SLUG" \
			--changelog="CHANGELOG.md" \
			--git="../.git"
			;;
esac
