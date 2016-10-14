// Copyright 2016 rvasm Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use nom::{IResult, space, digit};
use std::str;

use ast::{Instruction, Register};

named!(pub instructions<Vec<Instruction> >,
    separated_list!(terminated!(opt!(space), tag!("\n")), instruction)
);

named!(pub instruction<Instruction>,
    alt!(
        instruction_addi |
        instruction_slti |
        instruction_sltiu |
        instruction_andi |
        instruction_ori |
        instruction_xori |
        instruction_slli |
        instruction_srli |
        instruction_srai |
        instruction_nop |
        instruction_add
    )
);

macro_rules! instruction_immediate {
    ($name:ident, $opcode:expr, $t:path) => {
        named!($name<Instruction>,
            chain!(
                tag!($opcode) ~ space ~
                dest: register ~ tag!(",") ~ opt!(space) ~
                src: register ~ tag!(",") ~ opt!(space) ~
                imm: number,
                || $t(dest, src, imm)
            )
        );
    }
}

instruction_immediate!(instruction_addi, "addi", Instruction::Addi);
instruction_immediate!(instruction_slti, "slti", Instruction::Slti);
instruction_immediate!(instruction_sltiu, "sltiu", Instruction::Sltiu);
instruction_immediate!(instruction_andi, "andi", Instruction::Andi);
instruction_immediate!(instruction_ori, "ori", Instruction::Ori);
instruction_immediate!(instruction_xori, "xori", Instruction::Xori);
instruction_immediate!(instruction_slli, "slli", Instruction::Slli);
instruction_immediate!(instruction_srli, "srli", Instruction::Srli);
instruction_immediate!(instruction_srai, "srai", Instruction::Srai);

named!(instruction_nop<Instruction>, map!(tag!("nop"), |_| Instruction::Nop));
named!(instruction_add<Instruction>,
    chain!(
        tag!("add") ~
        space ~
        dest: register ~
        tag!(",") ~ opt!(space) ~
        src1: register ~
        tag!(",") ~ opt!(space) ~
        src2: register,
        || Instruction::Add(dest, src1, src2)
    )
);

named!(register<Register>,
    alt!(
        map!(alt!(tag!("x0") | tag!("zero")), |_| Register::X0) |
        map!(alt!(tag!("x1") | tag!("ra")), |_| Register::X1) |
        map!(alt!(tag!("x2") | tag!("sp")), |_| Register::X2) |
        map!(alt!(tag!("x3") | tag!("gp")), |_| Register::X3) |
        map!(alt!(tag!("x4") | tag!("tp")), |_| Register::X4) |
        map!(alt!(tag!("x5") | tag!("t0")), |_| Register::X5) |
        map!(alt!(tag!("x6") | tag!("t1")), |_| Register::X6) |
        map!(alt!(tag!("x7") | tag!("t2")), |_| Register::X7) |
        map!(alt!(tag!("x8") | tag!("s0") | tag!("fp")), |_| Register::X8) |
        map!(alt!(tag!("x9") | tag!("s1")), |_| Register::X9) |
        map!(alt!(tag!("x10") | tag!("a0")), |_| Register::X10) |
        map!(alt!(tag!("x11") | tag!("a1")), |_| Register::X11) |
        map!(alt!(tag!("x12") | tag!("a2")), |_| Register::X12) |
        map!(alt!(tag!("x13") | tag!("a3")), |_| Register::X13) |
        map!(alt!(tag!("x14") | tag!("a4")), |_| Register::X14) |
        map!(alt!(tag!("x15") | tag!("a5")), |_| Register::X15) |
        map!(alt!(tag!("x16") | tag!("a6")), |_| Register::X16) |
        map!(alt!(tag!("x17") | tag!("a7")), |_| Register::X17) |
        map!(alt!(tag!("x18") | tag!("s2")), |_| Register::X18) |
        map!(alt!(tag!("x19") | tag!("s3")), |_| Register::X19) |
        map!(alt!(tag!("x20") | tag!("s4")), |_| Register::X20) |
        map!(alt!(tag!("x21") | tag!("s5")), |_| Register::X21) |
        map!(alt!(tag!("x22") | tag!("s6")), |_| Register::X22) |
        map!(alt!(tag!("x23") | tag!("s7")), |_| Register::X23) |
        map!(alt!(tag!("x24") | tag!("s8")), |_| Register::X24) |
        map!(alt!(tag!("x25") | tag!("s9")), |_| Register::X25) |
        map!(alt!(tag!("x26") | tag!("s10")), |_| Register::X26) |
        map!(alt!(tag!("x27") | tag!("s11")), |_| Register::X27) |
        map!(alt!(tag!("x28") | tag!("t3")), |_| Register::X28) |
        map!(alt!(tag!("x29") | tag!("t4")), |_| Register::X29) |
        map!(alt!(tag!("x30") | tag!("t5")), |_| Register::X30) |
        map!(alt!(tag!("x31") | tag!("t6")), |_| Register::X31)
    )
);

named!(number<usize>, alt!(hex_number | decimal_number));
named!(hex_number<usize>,
    preceded!(
        tag!("0x"),
        map_opt!(digit, |bytes| {
            str::from_utf8(bytes).ok().and_then(|s| usize::from_str_radix(s, 16).ok())
        })
    )
);
named!(decimal_number<usize>,
   map_opt!(digit, |bytes| {
       str::from_utf8(bytes).ok().and_then(|s| usize::from_str_radix(s, 10).ok())
   })
);
