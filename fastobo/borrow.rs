//! Enhanced `Borrow`, `ToOwned` and `Cow` to use in tree structures.
//!
//! The main differences with traits from `std::borrow` are that:
//! - `Borrow::borrow` does not need to return a reference.
//! - `Borrow::borrow` has access to the lifetime of the referenced owner.
//! - `impl ToOwned<Owned=T> for B for` does not require `impl Borrow<B> for T`.
//! - `Cow` needs explicit references (e.g. `Cow<'a, &'a str>`).
//!
//! This allow a `Cow`-like behaviour on enum structures referencing pointers.

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use self::Cow::*;

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
        Cow::Borrowed(b)
    }
}

// impl<'a, B> Cow<'a, B>
// where
//     B: ToOwned<'a>,
//     <B as ToOwned<'a>>::Owned: 'a,
// {
//     /// Extracts the owned data.
//     ///
//     /// Clones the data if it is not already owned.
//     ///
//     /// # Examples
//     ///
//     /// Calling `into_owned` on a `Cow::Borrowed` clones the underlying data
//     /// and becomes a `Cow::Owned`:
//     ///
//     /// ```
//     /// use std::borrow::Cow;
//     ///
//     /// let s = "Hello world!";
//     /// let cow = Cow::Borrowed(s);
//     ///
//     /// assert_eq!(
//     ///   cow.into_owned(),
//     ///   String::from(s)
//     /// );
//     /// ```
//     ///
//     /// Calling `into_owned` on a `Cow::Owned` is a no-op:
//     ///
//     /// ```
//     /// use std::borrow::Cow;
//     ///
//     /// let s = "Hello world!";
//     /// let cow: Cow<str> = Cow::Owned(String::from(s));
//     ///
//     /// assert_eq!(
//     ///   cow.into_owned(),
//     ///   String::from(s)
//     /// );
//     /// ```
//     pub fn into_owned(self) -> <B as ToOwned<'a>>::Owned {
//         match self {
//             Borrowed(borrowed) => {
//                 *(&borrowed.to_owned() as *const _)
//             },
//             Owned(owned) => owned,
//         }
//     }
// }

impl<'a, B> Display for Cow<'a, B>
where
    B: Display + ToOwned<'a>,
    <B as ToOwned<'a>>::Owned: Display
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Borrowed(ref b) => b.fmt(f),
            Owned(ref o) => o.fmt(f),
        }
    }
}

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
