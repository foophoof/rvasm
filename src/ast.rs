// Copyright 2016 rvasm Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use encoding;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Addi(Register, Register, usize),
    Slti(Register, Register, usize),
    Sltiu(Register, Register, usize),
    Andi(Register, Register, usize),
    Ori(Register, Register, usize),
    Xori(Register, Register, usize),
    Slli(Register, Register, usize),
    Srli(Register, Register, usize),
    Srai(Register, Register, usize),
    Nop,
    Add(Register, Register, Register),
}

impl Instruction {
    pub fn to_raw(&self) -> u32 {
        use self::Instruction::*;
        match *self {
            Addi(dest, src, imm) => {
                encoding::I {
                    opcode: 0b0010011,
                    rd: dest.to_num(),
                    funct3: 0b000,
                    rs1: src.to_num(),
                    immediate: imm as i32,
                }.to_raw()
            }
            Slti(dest, src, imm) => {
                encoding::I {
                    opcode: 0b0010011,
                    rd: dest.to_num(),
                    funct3: 0b010,
                    rs1: src.to_num(),
                    immediate: imm as i32,
                }.to_raw()
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Register {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    X31,
}

impl Register {
    fn to_num(&self) -> u8 {
        use self::Register::*;
        match *self {
            X0 => 0,
            X1 => 1,
            X2 => 2,
            X3 => 3,
            X4 => 4,
            X5 => 5,
            X6 => 6,
            X7 => 7,
            X8 => 8,
            X9 => 9,
            X10 => 10,
            X11 => 11,
            X12 => 12,
            X13 => 13,
            X14 => 14,
            X15 => 15,
            X16 => 16,
            X17 => 17,
            X18 => 18,
            X19 => 19,
            X20 => 20,
            X21 => 21,
            X22 => 22,
            X23 => 23,
            X24 => 24,
            X25 => 25,
            X26 => 26,
            X27 => 27,
            X28 => 28,
            X29 => 29,
            X30 => 30,
            X31 => 31,
        }
    }
}
