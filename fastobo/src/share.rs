//! Enhanced `Borrow`, `ToOwned` and `Cow` to use in tree structures.
//!
//! This module define two traits:
//! - `Share`, related to `std::borrow::Borrow`
//! - `Redeem`, related to `std::borrow::ToOwned`
//! as well as the `Cow` enum with an interface close to `std::borrow::Cow`.
//!
//! The main differences with traits from `std::borrow` are that:
//! - `Share::share` does not need to return a reference.
//! - `Share::share` has access to the lifetime of the referenced owner.
//! - `Redeem<Owned=T> for B` does not require `impl Share<B> for T`
//! - `Cow` supports owning structures (preferably `Share`d ones).
//! - `Cow` needs explicit references where applicable (e.g. `Cow<'a, &'a str>`).
//!
//! This allow a `Cow`-like behaviour on enum structures referencing pointers.
//!
//! # Examples
//!
//! ```rust
//! # extern crate fastobo;
//! use fastobo::share::{Cow, Redeem, Share};
//!
//! pub struct MyOwner(String);
//!
//! #[derive(Clone)]
//! pub struct MyRef<'a>(Cow<'a, &'a str>);
//!
//! impl<'a> Share<'a, MyRef<'a>> for MyOwner {
//!     fn share(&'a self) -> MyRef<'a> {
//!         MyRef(Cow::from(self.0.as_ref()))
//!     }
//! }
//!
//! impl<'a> Redeem<'a> for MyRef<'a> {
//!     type Owned = MyOwner;
//!     fn redeem(&'a self) -> Self::Owned {
//!         MyOwner(self.0.to_string())
//!     }
//! }
//!
//! # let s1 = MyOwner(String::from("abc"));
//! # let s2 = s1.share().redeem();
//! # assert_eq!(s1.0, s2.0);
//! ```


use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;

// --- Share -----------------------------------------------------------------

/// A trait for obtaining a data view from a reference to an owned `struct`.
pub trait Share<'a, Shared: 'a> {
    fn share(&'a self) -> Shared;
}

impl<'a> Share<'a, &'a str> for String {
    fn share(&'a self) -> &'a str {
        &self.as_ref()
    }
}

// --- Redeem ----------------------------------------------------------------

/// A trait for taking ownership over viewed data with possibly expensives costs.
pub trait Redeem<'a>
where
    Self: 'a,
{
    type Owned: 'a;

    #[must_use = "cloning is often expensive and is not expected to have side effects"]
    fn redeem(&'a self) -> Self::Owned;
}

// impl<'a, Borrowed> ToOwned<'a> for &'a Borrowed
// where
//     Borrowed: std::borrow::ToOwned,
//     <Borrowed as std::borrow::ToOwned>::Owned: std::borrow::Borrow<Borrowed>,
// {
//     type Owned = <&'a Borrowed as std::borrow::ToOwned>::Owned;
//     fn to_owned(&'a self) -> Self::Owned {
//         <Self as std::borrow::ToOwned>::to_owned(self)
//     }
// }

impl<'a, T> Redeem<'a> for &'a T
where
    T: Clone
{
    type Owned = T;
    fn redeem(&self) -> Self::Owned {
        (*self).clone()
    }
}

impl<'a> Redeem<'a> for &'a str {
    type Owned = String;
    fn redeem(&'a self) -> String {
        self.to_string()
    }
}

// --- Cow -------------------------------------------------------------------

pub enum Cow<'a, B>
where
    B: 'a + Redeem<'a>,
{
    /// Borrowed data.
    Borrowed(B),
    /// Owned data.
    Owned(<B as Redeem<'a>>::Owned),
}

// FIXME:
// impl<'a, B> Cow<'a, &'a B>
// where
//     B: 'a + Clone + Share<'a, &'a B>,
// {
//     pub fn into_owned(self) -> <&'a B as Redeem<'a>>::Owned {
//         match self {
//             Borrowed(b) => b.to_owned(),
//             Owned(o) => o,
//         }
//     }
// }

impl<'a, B> Share<'a, B> for Cow<'a, B>
where
    B: Redeem<'a> + Clone,
    <B as Redeem<'a>>::Owned: Share<'a, B>,
{
    fn share(&'a self) -> B {
        match self {
            Cow::Borrowed(b) => b.clone(),
            Cow::Owned(o) => o.share(),
        }
    }
}

impl<'a, B> Redeem<'a> for Cow<'a, B>
where
    B: Redeem<'a>,
    <B as Redeem<'a>>::Owned: Clone,
{
    type Owned = <B as Redeem<'a>>::Owned;
    fn redeem(&'a self) -> <Self as Redeem<'a>>::Owned {
        match self {
            Cow::Borrowed(b) => b.redeem(),
            Cow::Owned(c) => c.clone(),
        }
    }
}

impl<'a, B> Clone for Cow<'a, B>
where
    B: Clone + Redeem<'a>,
    <B as Redeem<'a>>::Owned: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Cow::Borrowed(b) => Cow::Borrowed(b.clone()),
            Cow::Owned(b) => Cow::Owned(b.clone()),
        }
    }
}

impl<'a, B> From<&'a B> for Cow<'a, &'a B>
where
    &'a B: Redeem<'a>
{
    fn from(b: &'a B) -> Self {
        Cow::Borrowed(b)
    }
}

impl<'a, B> Display for Cow<'a, B>
where
    B: Display + Redeem<'a>,
    <B as Redeem<'a>>::Owned: Display
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Cow::Borrowed(b) => b.fmt(f),
            Cow::Owned(o) => o.fmt(f),
        }
    }
}

impl<'a, B> Debug for Cow<'a, B>
where
    B: Debug + Redeem<'a>,
    <B as Redeem<'a>>::Owned: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Cow::Borrowed(b) => b.fmt(f),
            Cow::Owned(o) => o.fmt(f),
        }
    }
}

impl<'a, B> Hash for Cow<'a, B>
where
    B: Hash + Redeem<'a>,
    <B as Redeem<'a>>::Owned: Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher
    {
        match self {
            Cow::Borrowed(b) => b.hash(state),
            Cow::Owned(o) => o.hash(state),
        };
    }
}

// --- Cow<str> --------------------------------------------------------------

impl<'a> Cow<'a, &'a str> {
    fn as_str(&self) -> &str {
        match self {
            Cow::Borrowed(s) => *s,
            Cow::Owned(s) => s.as_str(),
        }
    }
}

impl<'a> Deref for Cow<'a, &'a str> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            Cow::Borrowed(s) => Clone::clone(s),
            Cow::Owned(ref s) => s.as_ref(),
        }
    }
}

impl<'a> Eq for Cow<'a, &'a str> {}

impl<'a> From<&'a str> for Cow<'a, &'a str> {
    fn from(s: &'a str) -> Self {
        Cow::Borrowed(s)
    }
}

impl<'a> From<String> for Cow<'a, &'a str> {
    fn from(s: String) -> Self {
        Cow::Owned(s)
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

// --- Cow<[T]> --------------------------------------------------------------

impl<'a, T> Redeem<'a> for &'a [T]
where
    T: 'a + Clone
{
    type Owned = Vec<T>;
    fn redeem(&'a self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.len());
        v.clone_from_slice(self);
        v
    }
}
