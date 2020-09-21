#![deny(warnings)]
#![cfg_attr(not(feature="std"), no_std)]

#[cfg(feature="std")]
extern crate core;

use core::marker::PhantomData;
use educe::Educe;
#[cfg(feature="std")]
use std::panic::{UnwindSafe, RefUnwindSafe};

/// A [`PhantomData`](core::marker::PhantomData) analog which prevents "parameter is never used" error,
/// but does not produce any restrictions in contrast with `PhantomData`.
#[derive(Educe)]
#[educe(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash, Default, Debug)]
pub struct PhantomType<T: ?Sized>(PhantomData<T>);

unsafe impl<T: ?Sized> Send for PhantomType<T> { }
unsafe impl<T: ?Sized> Sync for PhantomType<T> { }
impl<T: ?Sized> Unpin for PhantomType<T> { }

#[cfg(feature="std")]
impl<T: ?Sized> RefUnwindSafe for PhantomType<T> { }

#[cfg(feature="std")]
impl<T: ?Sized> UnwindSafe for PhantomType<T> { }
