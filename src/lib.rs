// MIT + Apache 2.0

//! Rust atomic primitives that can be configured to not be atomic.

#![forbid(unsafe_code)]
#![warn(rust_2018_idioms)]
#![no_std]

#[cfg(not(feature = "atomic"))]
use core::cell::Cell;
#[cfg(feature = "atomic")]
use core::sync::atomic::{
    AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicU16, AtomicU32,
    AtomicU64, AtomicU8, AtomicUsize,
};

use core::sync::atomic::Ordering;
use doc_comment::doc_comment;

macro_rules! maybe_atomic_type {
    ($tyname: ident: $atomic: ty | $unsync: ty) => {
        doc_comment! {
            concat!(
                "An atomic structure that wraps either an ",
                stringify!($atomic),
                " or a ",
                stringify!($unsync),
                ", depending on if atomics are available."
            ),
            #[repr(transparent)]
            pub struct $tyname {
                #[cfg(feature = "atomic")]
                atomic: $atomic,
                #[cfg(not(feature = "atomic"))]
                unsync: Cell<$unsync>,
            }
        }

        impl $tyname {
            doc_comment! {
                concat!(
                    "Creates a new instance of ",
                    stringify!($tyname),
                    "."
                ),
                #[inline]
                pub fn new(inner: $unsync) -> Self {
                    Self::new_impl(inner)
                }
            }

            #[cfg(feature = "atomic")]
            #[inline]
            fn new_impl(inner: $unsync) -> Self {
                Self {
                    atomic: <$atomic>::new(inner),
                }
            }

            #[cfg(not(feature = "atomic"))]
            #[inline]
            fn new_impl(inner: $unsync) -> Self {
                Self {
                    unsync: Cell::new(inner),
                }
            }

            /// Get a mutable reference to the value contained within.
            #[inline]
            pub fn get_mut(&mut self) -> &mut $unsync {
                self.get_mut_impl()
            }

            #[cfg(feature = "atomic")]
            #[inline]
            fn get_mut_impl(&mut self) -> &mut $unsync {
                self.atomic.get_mut()
            }

            #[cfg(not(feature = "atomic"))]
            #[inline]
            fn get_mut_impl(&mut self) -> &mut $unsync {
                self.unsync.get_mut()
            }

            /// Copy the value out of this container using the specified ordering.
            #[inline]
            pub fn load(&self, order: Ordering) -> $unsync {
                self.load_impl(order)
            }

            #[cfg(feature = "atomic")]
            #[inline]
            fn load_impl(&self, order: Ordering) -> $unsync {
                self.atomic.load(order)
            }

            #[cfg(not(feature = "atomic"))]
            #[inline]
            fn load_impl(&self, _order: Ordering) -> $unsync {
                self.unsync.get()
            }

            /// Store a value in this container.
            #[inline]
            pub fn store(&self, val: $unsync, order: Ordering) {
                self.store_impl(val, order);
            }

            #[cfg(feature = "atomic")]
            #[inline]
            fn store_impl(&self, val: $unsync, order: Ordering) {
                self.atomic.store(val, order);
            }

            #[cfg(not(feature = "atomic"))]
            #[inline]
            fn store_impl(&self, val: $unsync, _order: Ordering) {
                self.unsync.set(val);
            }

            /// Swap two values, returning the old value stored in this container.
            #[inline]
            pub fn swap(&self, val: $unsync, order: Ordering) -> $unsync {
                self.swap_impl(val, order)
            }

            #[cfg(feature = "atomic")]
            #[inline]
            fn swap_impl(&self, val: $unsync, order: Ordering) -> $unsync {
                self.atomic.swap(val, order)
            }

            #[cfg(not(feature = "atomic"))]
            #[inline]
            fn swap_impl(&self, val: $unsync, _order: Ordering) -> $unsync {
                self.unsync.replace(val)
            }
        }
    };
}

maybe_atomic_type! {MaybeAtomicBool: AtomicBool | bool}
maybe_atomic_type! {MaybeAtomicU8: AtomicU8 | u8}
maybe_atomic_type! {MaybeAtomicU16: AtomicU16 | u16}
maybe_atomic_type! {MaybeAtomicU32: AtomicU32 | u32}
maybe_atomic_type! {MaybeAtomicU64: AtomicU64 | u64}
maybe_atomic_type! {MaybeAtomicUsize: AtomicUsize | usize}
maybe_atomic_type! {MaybeAtomicI8: AtomicI8 | i8}
maybe_atomic_type! {MaybeAtomicI16: AtomicI16 | i16}
maybe_atomic_type! {MaybeAtomicI32: AtomicI32 | i32}
maybe_atomic_type! {MaybeAtomicI64: AtomicI64 | i64}
maybe_atomic_type! {MaybeAtomicIsize: AtomicIsize | isize}
