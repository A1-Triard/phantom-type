#![deny(warnings)]
#![cfg_attr(not(feature="std"), no_std)]

#[cfg(feature="std")]
extern crate core;

use core::marker::PhantomData;
use educe::Educe;
#[cfg(feature="std")]
use std::panic::{UnwindSafe, RefUnwindSafe};

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
