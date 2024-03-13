#![deny(warnings)]
#![doc(test(attr(deny(warnings))))]
#![doc(test(attr(allow(dead_code))))]
#![doc(test(attr(allow(unused_variables))))]

#![cfg_attr(not(feature="std"), no_std)]

//! **Crate features**
//!
//! * `"std"`
//! Enabled by default. Disable to make the library `#![no_std]`.

#[cfg(feature="std")]
extern crate core;

use core::marker::PhantomData;
use educe::Educe;
#[cfg(feature="std")]
use std::panic::{UnwindSafe, RefUnwindSafe};

/// A [`PhantomData`] analog which prevents "parameter is never used" error,
/// but does not produce any restrictions in contrast with `PhantomData`.
#[derive(Educe)]
#[educe(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash, Default, Debug)]
pub struct PhantomType<T: ?Sized>(PhantomData<T>);

impl<T: ?Sized> PhantomType<T> {
    /// Creates `PhantomType` instance.
    pub const fn new() -> Self { PhantomType(PhantomData) }
}

unsafe impl<T: ?Sized> Send for PhantomType<T> { }
unsafe impl<T: ?Sized> Sync for PhantomType<T> { }
impl<T: ?Sized> Unpin for PhantomType<T> { }

#[cfg(feature="std")]
impl<T: ?Sized> RefUnwindSafe for PhantomType<T> { }

#[cfg(feature="std")]
impl<T: ?Sized> UnwindSafe for PhantomType<T> { }
