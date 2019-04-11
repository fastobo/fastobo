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

release_py() {

	log Building fastobo-py sdist
	$PYTHON setup.py sdist

	log Building fastobo-py wheel
	if [ "$TRAVIS_OS_NAME" = "linux" ]; then
		IMG="quay.io/pypa/manylinux1_x86_64"
		CP=cp$(echo $TRAVIS_PYTHON_VERSION | sed 's/\.//')
		docker run --rm -v $TRAVIS_BUILD_DIR:/io $IMG /io/ci/build-wheels.sh $CP
	else
		$PYTHON setup.py bdist_wheel
	fi

	log Publishing fastobo-py $($PYTHON setup.py --version)
	twine upload --skip-existing dist/*.whl dist/*.tar.gz
}


# --- Publish crate to `crates.io` and/or `pypi.org` -------------------------

case "$TRAVIS_TAG" in
	# Release fastobo-syntax
	v*-syntax)
		if [ "$TRAVIS_BUILD_STAGE_NAME" = "Rust" ]; then
			cd "$TRAVIS_BUILD_DIR/fastobo-syntax"
			log Publishing fastobo-syntax ${TRAVIS_TAG%-syntax}
			cargo publish --token $CRATES_IO_TOKEN
		fi
		;;
	# Release fastobo
	v*)
		if [ "$TRAVIS_BUILD_STAGE_NAME" = "Rust" ]; then
			cd "$TRAVIS_BUILD_DIR/fastobo"
			log Publishing fastobo $TRAVIS_TAG
			cargo publish --token $CRATES_IO_TOKEN
		fi
		;;
	# Release fastobo-py
	v*-py)
		if [ "$TRAVIS_BUILD_STAGE_NAME" != "Rust" ]; then
			release_py
		fi
		;;
	# Release dev version of `fastobo-py`
	*)
		if [ "$TRAVIS_BUILD_STAGE_NAME" != "Rust" ]; then
			VERSION=$($PYTHON setup.py --version)-dev$(git rev-list --count --all)
			sed -i'.BAK' -e "s/version = $($PYTHON setup.py --version)/version = $VERSION/g" setup.cfg
			release_py
		fi
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
