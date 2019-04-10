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

log Installing Rust nightly
curl -sSf https://build.travis-ci.org/files/rustup-init.sh > ~/rustup-init.sh
sh ~/rustup-init.sh --default-toolchain=nightly -y
export PATH="$HOME/.cargo/bin:$PATH"
export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$HOME/.cargo/lib"

# Compile wheels

for PYBIN in /opt/python/${1}*/bin; do
    export PYTHON_SYS_EXECUTABLE="$PYBIN/python"
    export PYTHON_LIB=$(${PYBIN}/python -c "import sysconfig; print(sysconfig.get_config_var('LIBDIR'))")
    export LIBRARY_PATH="$LIBRARY_PATH:$PYTHON_LIB"
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$PYTHON_LIB"
    "${PYBIN}/pip" install -U setuptools setuptools-rust
    "${PYBIN}/pip" wheel /io/ -w /dist/
done

# Bundle external shared libraries into the wheels
for whl in /dist/*.whl; do
    auditwheel repair "$whl" -w /io/dist/
done
