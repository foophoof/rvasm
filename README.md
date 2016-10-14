# rvasm

rvasm is a RISC-V assembler.

## Usage

    $ rvasm <<< "addi x0,x1,17" > foo.o
    $ riscv32-unknown-elf-objdump -D -b binary -mriscv:rv32 foo.o
    [...]
    00000000 <.data>:
       0:   13801001                addi    zero,ra,17

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
