Call assembly from rust
=======================


This is a small working demo of how to include assembler code from Rust.

Source: https://stackoverflow.com/questions/47455163/how-to-call-assembly-functions-inside-rust

Note:  We are compiling and running code from a separate assembler file.  For inline assembler see: https://doc.rust-lang.org/1.8.0/book/inline-assembly.html

## Usage:

Assuming that you have rustc and cargo installed, you can build and run with:


    cargo run

The linker line is most informative about how this works.
