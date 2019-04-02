//! Enhanced `Borrow`, `ToOwned` and `Cow` to use in tree structures.
//!
//! The main differences with traits from `std::borrow` are that:
//! - `Borrow::borrow` does not need to return a reference.
//! - `Borrow::borrow` has access to the lifetime of the referenced owner.
//! - `impl ToOwned<Owned=T> for B for` does not require `impl Borrow<B> for T`.
//! - `Cow` needs explicit references where (e.g. `Cow<'a, &'a str>`).
//!
//! This allow a `Cow`-like behaviour on enum structures referencing pointers.
//!
//! # Examples
//!
//! ```rust
//! # extern crate fastobo;
//! use fastobo::borrow::{Borrow, Cow, ToOwned};
//!
//! pub struct MyOwner(String);
//!
//! #[derive(Clone)]
//! pub struct MyRef<'a>(Cow<'a, &'a str>);
//!
//! impl<'a> Borrow<'a, MyRef<'a>> for MyOwner {
//!     fn borrow(&'a self) -> MyRef<'a> {
//!         MyRef(Cow::from(self.0.as_ref()))
//!     }
//! }
//!
//! impl<'a> ToOwned<'a> for MyRef<'a> {
//!     type Owned = MyOwner;
//!     fn to_owned(&'a self) -> Self::Owned {
//!         MyOwner(self.0.to_string())
//!     }
//! }
//! ```


use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;

use self::Cow::*;

// --- Borrow ----------------------------------------------------------------

pub trait Borrow<'a, Borrowed: 'a> {
    fn borrow(&'a self) -> Borrowed;
}

impl<'a, T, B> Borrow<'a, &'a B> for T
where
    T: std::borrow::Borrow<B>,
{
    fn borrow(&'a self) -> &'a B {
        <Self as std::borrow::Borrow<B>>::borrow(self)
    }
}

impl<'a> Borrow<'a, &'a str> for String {
    fn borrow(&'a self) -> &'a str {
        &self.as_ref()
    }
}

// --- ToOwned ---------------------------------------------------------------

pub trait ToOwned<'a>
where
    Self: 'a,
{
    type Owned: 'a;

    #[must_use = "cloning is often expensive and is not expected to have side effects"]
    fn to_owned(&'a self) -> Self::Owned;
}

impl<'a, Borrowed> ToOwned<'a> for &'a Borrowed
where
    Borrowed: std::borrow::ToOwned,
    <Borrowed as std::borrow::ToOwned>::Owned: std::borrow::Borrow<Borrowed>,
{
    type Owned = <&'a Borrowed as std::borrow::ToOwned>::Owned;
    fn to_owned(&'a self) -> Self::Owned {
        <Self as std::borrow::ToOwned>::to_owned(self)
    }
}

impl<'a> ToOwned<'a> for &'a str {
    type Owned = String;
    fn to_owned(&'a self) -> String {
        self.to_string()
    }
}

// --- Cow -------------------------------------------------------------------

pub enum Cow<'a, B: 'a>
where
    B: ToOwned<'a>,
{
    /// Borrowed data.
    Borrowed(B),
    /// Owned data.
    Owned(<B as ToOwned<'a>>::Owned),
}

impl<'a, B> ToOwned<'a> for Cow<'a, B>
where
    B: ToOwned<'a>,
    <B as ToOwned<'a>>::Owned: Clone,
{
    type Owned = <B as ToOwned<'a>>::Owned;
    fn to_owned(&'a self) -> <Cow<'a, B> as ToOwned>::Owned {
        match self {
            Cow::Borrowed(b) => b.to_owned(),
            Cow::Owned(c) => c.clone(),
        }
    }
}

impl<'a, B> Clone for Cow<'a, B>
where
    B: Clone + ToOwned<'a>,
    <B as ToOwned<'a>>::Owned: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Borrowed(b) => Borrowed(b.clone()),
            Owned(b) => Owned(b.clone()),
        }
    }
}

impl<'a, B> From<&'a B> for Cow<'a, &'a B>
where
    &'a B: ToOwned<'a>
{
    fn from(b: &'a B) -> Self {
        Borrowed(b)
    }
}

impl<'a, B> Display for Cow<'a, B>
where
    B: Display + ToOwned<'a>,
    <B as ToOwned<'a>>::Owned: Display
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Borrowed(b) => b.fmt(f),
            Owned(o) => o.fmt(f),
        }
    }
}

impl<'a, B> Debug for Cow<'a, B>
where
    B: Debug + ToOwned<'a>,
    <B as ToOwned<'a>>::Owned: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Borrowed(b) => b.fmt(f),
            Owned(o) => o.fmt(f),
        }
    }
}

impl<'a, B> Hash for Cow<'a, B>
where
    B: Hash + ToOwned<'a>,
    <B as ToOwned<'a>>::Owned: Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher
    {
        match self {
            Borrowed(b) => b.hash(state),
            Owned(o) => o.hash(state),
        }
    }
}

// --- Cow<str> --------------------------------------------------------------

impl<'a> Cow<'a, &'a str> {
    fn as_str(&self) -> &str {
        match self {
            Borrowed(s) => *s,
            Owned(s) => s.as_str(),
        }
    }
}

impl<'a> Deref for Cow<'a, &'a str> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            Borrowed(s) => Clone::clone(s),
            Owned(ref s) => s.as_ref(),
        }
    }
}

impl<'a> Eq for Cow<'a, &'a str> {}

impl<'a> From<&'a str> for Cow<'a, &'a str> {
    fn from(s: &'a str) -> Self {
        Borrowed(s)
    }
}

impl<'a> From<String> for Cow<'a, &'a str> {
    fn from(s: String) -> Self {
        Owned(s)
    }
}

impl<'a> PartialEq<str> for Cow<'a, &'a str> {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl<'a> PartialEq for Cow<'a, &'a str> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}


// --- Cow<[T]>

impl<'a, T> ToOwned<'a> for &'a [T]
where
    T: 'a + Clone
{
    type Owned = Vec<T>;
    fn to_owned(&'a self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.len());
        v.clone_from_slice(self);
        v
    }
}
