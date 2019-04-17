#!/bin/sh

. $(dirname $(dirname $0))/functions.sh

# --- Deploy coverage --------------------------------------------------------

curl -SsL "https://codecov.io/bash" | bash
