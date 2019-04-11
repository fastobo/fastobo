#![recursion_limit="128"]
#![allow(unused_imports)]

extern crate fastobo;
extern crate pyo3;
extern crate libc;
extern crate url;

#[macro_use]
extern crate opaque_typedef_macros;
extern crate opaque_typedef;

#[macro_use]
extern crate fastobo_py_derive;

pub mod utils;
pub mod py;
pub mod pyfile;
