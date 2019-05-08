#!/bin/sh -e

. $(dirname $(dirname $0))/functions.sh

# --- Patch version number ---------------------------------------------------

case "$TRAVIS_TAG" in
  v*-syntax)  ;;
  v*-py)  ;;
  v*) ;;
  *)
    VERSION=$(python setup.py --version)-dev$(git rev-list --count --all)
    sed -i'.BAK' -e "s/version = $(python setup.py --version)/version = $VERSION/g" setup.cfg
esac

# --- Wheels -----------------------------------------------------------------

log Building wheel
CP=cp$(echo $TRAVIS_PYTHON_VERSION | sed 's/\.//')
docker exec -it manylinux sh /io/ci/travis/manylinux/_after_success.sh $CP

# --- Deploy -----------------------------------------------------------------

case "$TRAVIS_TAG" in
	v*-syntax) ;;
	v*-py) twine upload --skip-existing dist/*.whl dist/*.tar.gz ;;
	v*)  ;;
	*) twine upload --skip-existing dist/*.whl dist/*.tar.gz ;;
esac
