//! Defines `Relevant` type to use in types that requires
//! custom destruction.
//!
//! With default feature "std" it `Drop` implementation will not trigger panic
//! in case of unwinding (e.g. already panicking).
//! 

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
use core as std;

/// Values of this type can't be automatically dropped.
/// If struct or enum has field with type `Relevant`,
/// it can't be automatically dropped either. And so considered relevant too.
/// User has to deconstruct such values and call `Relevant::dispose`.
/// If relevant field is private it means that user has to move value into some public method.
/// For example `memory::Block` should be returned to the `MemoryAllocator` it came from.
/// 
/// User of the engine won't usually deal with real relevant types.
/// More often user will face wrappers that has backdoor - some technique
/// to dispose internal relevant fields with runtime cost.
/// In debug mode such wrappers can put warnings in log.
/// So that user will know they should be disposed manually.
/// 
/// # Panics
/// 
/// Panics when dropped unless:
/// * `log` feature is enabled. It this case it emmits `log::error!`.
/// * `std` feature is enabled and thread is already in panicking state.
/// 
#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde-1", derive(serde::Serialize, serde::Deserialize))]
pub struct Relevant;

impl Relevant {
    /// Dispose this value.
    pub fn dispose(self) {
        std::mem::forget(self)
    }
}

impl Drop for Relevant {
    fn drop(&mut self) {
        whine()
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "log")] {
        fn whine() {
            log::error!("Values of this type can't be dropped!")
        }
    } else if #[cfg(feature = "std")] {
        fn whine() {
            if !std::thread::panicking() {
                panic!("Values of this type can't be dropped!")
            }
        }
    } else {
        fn whine()  {
            panic!("Values of this type can't be dropped!")
        }
    }
}
