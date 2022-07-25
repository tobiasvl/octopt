octopt
======
[![crates.io](https://img.shields.io/crates/v/octopt.svg)](https://crates.io/crates/octopt)
[![docs.rs](https://img.shields.io/docsrs/octopt.svg)](https://docs.rs/octopt)
[![dependency status](https://deps.rs/repo/github/tobiasvl/octopt/status.svg)](https://deps.rs/crate/octopt)

`octopt` is a library for handling CHIP-8 configuration settings.

It contains Rust data structures that aim to represent all possible CHIP-8 options. It can also serialize and deserialize these structures from and into [Octo](https://github.com/JohnEarnest/Octo)'s standard JSON format and [C-Octo](https://github.com/JohnEarnest/c-octo)'s INI-like format.

See the [`octopt` documentation](https://docs.rs/octopt) for details. The documentation is fairly verbose.

Rationale
---------

CHIP-8 is a virtual machine for playing simple computer games written in interpreted bytecode. It has been around since 1977, and has many slightly incompatible implementations.

CHIP-8 games often require specific behavior from its emulator (actually, interpreter) to run correctly, but since it's impossible to know what behavior it expects just by looking at the game's bytecode, additional metadata is required to instruct the emulator how to run the game. This metadata can come in the form of JSON – as is the case with Octocarts (see [`decart`](https://crates.io/crates/decart)) and the [CHIP-8 Community Archive](https://github.com/JohnEarnest/chip8Archive) – or an [INI file](https://en.wikipedia.org/wiki/INI_file), as is the case for [C-Octo](https://github.com/JohnEarnest/c-octo).

Use cases
---------

There are many CHIP-8 emulators out there (and many made in Rust), but most only support the same set of a couple of dozen games made during the 90s because they followed a non-comprehensive tutorial. (Shameless plug: I've written [a tutorial that covers the important CHIP-8 varities out there](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/).)

If you're making a CHIP-8 emulator in Rust, you could use this library as a guide to what configuration options you should (or could) include to support most of the CHIP-8 games that have been made during the last 44 years. 

You could allow the user to provide a CHIP-8 game as well as an associated `.octo.rc` file (which C-Octo uses) with settings required for the game to run, and you could parse that file with `octopt`.

Or you could use the library [`decart`](https://crates.io/crates/decart), which uses `octopt`, in order to let users load Octocarts which come with the settings baked in with the game.

Let me know if you figure out a cool use case!

See also
--------

* [Octo](https://github.com/JohnEarnest/Octo) and [C-Octo](https://github.com/JohnEarnest/c-octo), integrated development environments for CHIP-8 games
* [CHIP-8 Community Archive](https://github.com/JohnEarnest/chip8Archive), a database of public domain CHIP-8 games and metadata
* [`decart`](https://crates.io/crates/decart), a Rust library for encoding and decoding CHIP-8 metadata from Octocarts
* [`deca`](https://crates.io/crates/deca), a CHIP-8 emulator that uses this library to support a wide variety of CHIP-8 games
* [`termin-8`](https://crates.io/crates/termin-8), a terminal frontend to `deca`
