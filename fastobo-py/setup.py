#!/usr/bin/env python3

import os

import setuptools
import setuptools_rust as rust

setuptools.setup(
    rust_extensions=[rust.RustExtension(
        "fastobo",
        path=os.path.join("fastobo", "Cargo.toml"),
        binding=rust.Binding.PyO3,
        strip=rust.Strip.Debug
    )],
)
