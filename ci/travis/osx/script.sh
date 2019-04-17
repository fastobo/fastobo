#!/bin/sh

. $(dirname $(dirname $0))/functions.sh

# --- Test -------------------------------------------------------------------

$PYTHON setup.py test
