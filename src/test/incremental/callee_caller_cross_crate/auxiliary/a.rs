// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-flags: -Z incremental-cc

#![crate_type="rlib"]

#[cfg(rpass1)]
pub fn function0(x: u32) -> u32 {
    x
}

#[cfg(rpass2)]
pub fn function0(x: i32) -> i32 {
    x
}

pub fn function1(x: u32) {
}
