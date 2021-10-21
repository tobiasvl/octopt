octopt
======
[![crates.io](https://img.shields.io/crates/v/octopt.svg)](https://crates.io/crates/octopt)
[![docs.rs](https://img.shields.io/docsrs/octopt.svg)](https://docs.rs/octopt)

`octopt` is a library for handling CHIP-8 configuration settings.

CHIP-8 is a virtual machine for playing simple computer games written in interpreted bytecode. It has been around since 1977, and has many slightly incompatible implementations.

CHIP-8 games often require specific behavior from its interpreter to run correctly, but since it's impossible to know what behavior it expects just by looking at the game's bytecode, additional metadata is required to instruct the interpreter how to run the game. This metadata can come in the form of JSON – as is the case with Octocarts (see [`decart`](https://crates.io/crates/decart)) and the [CHIP-8 Community Archive](https://github.com/JohnEarnest/chip8Archive) – or an [INI file](https://en.wikipedia.org/wiki/INI_file), as is the case for [C-Octo](https://github.com/JohnEarnest/c-octo).

This library contains Rust data structures that represent all possible CHIP-8 options. It can also serialize and deserialize these structures into the standard JSON and INI formats.

See also
--------

* [Octo](https://github.com/JohnEarnest/Octo) and [C-Octo](https://github.com/JohnEarnest/c-octo), integrated development environments for CHIP-8 games
* [`decart`](https://crates.io/crates/decart), a crate for encoding and decoding CHIP-8 metadata from Octocarts
* [`deca`](https://crates.io/crates/deca), a CHIP-8 interpreter that uses this library to support a wide variety of CHIP-8 games
* [`termin-8`](https://crates.io/crates/termin-8), a terminal frontend to `deca`
