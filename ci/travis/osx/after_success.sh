#!/bin/sh -e

. $(dirname $(dirname $0))/functions.sh

# --- Patch version number ---------------------------------------------------

case "$TRAVIS_TAG" in
  v*-syntax)  ;;
  v*-py)  ;;
  v*) ;;
  *)
    VERSION=$($PYTHON setup.py --version)-dev$(git rev-list --count --all)
    sed -i'.BAK' -e "s/version = $($PYTHON setup.py --version)/version = $VERSION/g" setup.cfg
esac

# --- Wheels -----------------------------------------------------------------

log Building wheel
$PYTHON setup.py sdist bdist_wheel

# --- Deploy -----------------------------------------------------------------

case "$TRAVIS_TAG" in
	v*-syntax) ;;
	v*-py) twine upload --skip-existing dist/*.whl dist/*.tar.gz ;;
  v*)  ;;
	*) twine upload --skip-existing dist/*.whl dist/*.tar.gz ;;
esac
