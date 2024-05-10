//! Implementation of `errno` functionality for wasm32 systems.
//!
//! Adapted from `unix.rs`

// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::str;

use crate::Errno;

pub fn with_description<F, T>(err: Errno, callback: F) -> T
where
    F: FnOnce(Result<&str, Errno>) -> T,
{
    // we have no way of obtaining a description...
    callback(Err(err))
}

pub const STRERROR_NAME: &str = "strerror_r";

pub fn errno() -> Errno {
    // not supported on this platform
    Errno(0)
}

pub fn set_errno(Errno(errno): Errno) {
    // no-op
}
