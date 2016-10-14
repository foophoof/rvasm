// Copyright 2016 rvasm Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[macro_use]
extern crate nom;

use std::io::prelude::*;
use std::io;

mod ast;
mod encoding;
mod parser;

fn main() {
    let mut buffer = Vec::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_end(&mut buffer).expect("couldn't read stdin");

    let (_, res) = parser::instructions(buffer.as_slice()).unwrap();
    let mut buf = [0; 4];
    for instruction in res {
        let raw = instruction.to_raw();
        buf[0] = (raw & 0x000000FF) as u8;
        buf[1] = ((raw & 0x0000FF00) >> 8) as u8;
        buf[2] = ((raw & 0x00FF0000) >> 16) as u8;
        buf[3] = ((raw & 0xFF000000) >> 24) as u8;

        io::stdout().write(&buf[..]).expect("couldn't write output");
    }
}
