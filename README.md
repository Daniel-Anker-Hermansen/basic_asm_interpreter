# Basic assembly interpreter

Interpreter for the basic assembly language invented by [Peyman Afshani](https://pure.au.dk/portal/da/persons/peyman%40cs.au.dk), used in the course, Computer architecture, network and operating systems.

## Usage
First argument is the source file. The file must follow the specification with one instruction per line. Comments can be written after `//`, `#` or `;`. Empty lines are ignored. At the end the state of the registers and zero flag are dumped. Additionally, it supports the instruction `debug` which dumps the registers and the zero flag and waits for the user to press enter to continue.

The registers at the beginning can be set with e.g. `r0=4`

For example running `test.s` with r0 set to 10 and r1 set to 20 uses the following arguments `test.s r0=10 r1=20`

## Install
Binaries for x86_64-linux, armv7-linux and x86_64-windows can be found [here](https://github.com/Daniel-Anker-Hermansen/basic_asm_interpreter/releases/tag/v1.1.0)

You can compile from source with cargo using `cargo build --release`
