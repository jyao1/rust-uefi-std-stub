#![no_std]

extern crate alloc;

pub use alloc::format;

pub mod io;
pub mod sys;
pub mod error;
pub mod time;

pub use alloc::boxed;
pub use alloc::rc;
pub use alloc::vec;
pub use alloc::string;
pub use alloc::borrow;

pub use core::cell;
pub use core::cmp;
pub use core::convert;
pub use core::default;
pub use core::fmt;
pub use core::mem;
pub use core::ops;
pub use core::slice;
pub use core::result;
pub use core::marker;
pub use core::option;
pub use core::iter;
pub use core::hash;
pub use core::str;

pub mod sync {

    // wrap spin::Mutex same as std::sync::Mutex
    // pub use spin::Mutex;
    pub struct Mutex<T:?Sized>(spin::Mutex<T>);
    impl<T> Mutex<T> {
        pub fn new(user_data: T) -> Self {
            Mutex(spin::Mutex::new(user_data))
        }
        pub fn try_lock(&self) -> Result<spin::MutexGuard<T>, ()>
        {
            if let Some(res) = self.0.try_lock() {
                Ok(res)
            } else {
                Err(())
            }
        }

        pub fn lock(&self) -> Result<spin::MutexGuard<T>, ()>
        {
            self.try_lock()
        }
    }
    pub use alloc::sync::*;

    pub use spin::MutexGuard;
}

pub mod collections {
    pub use alloc::collections::*;

    // use hashbrown's hashmap instead of std's hashmap
    pub use hashbrown::HashMap;
    pub use hashbrown::HashSet;
    pub use hashbrown::hash_map;
}

pub mod memchr {
    pub fn memrchr(_needle: u8, _haystack: &[u8]) -> Option<usize> {
        None
    }
    pub fn memchr(_needle: u8, _haystack: &[u8]) -> Option<usize> {
        None
    }
}

pub mod prelude;
