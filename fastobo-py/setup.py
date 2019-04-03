#!/usr/bin/env python3

import setuptools
import setuptools_rust as rust

setuptools.setup(
    rust_extensions=rust.find_rust_extensions(
        binding=rust.Binding.PyO3,
        strip=rust.Strip.Debug
    ),
    name="fastobo",
    version="0.1.0",
    # packages=["fastobo"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)
