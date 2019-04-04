#!/usr/bin/env python3

import setuptools
import setuptools_rust as rust

setuptools.setup(
    rust_extensions=rust.find_rust_extensions(
        binding=rust.Binding.PyO3,
        strip=rust.Strip.Debug
    ),
)
