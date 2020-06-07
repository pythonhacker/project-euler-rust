## About

The repo contains original solutions to [Project Euler][1] in Rust.

## Source files

Each source file maps to a given Project Euler problem by index. So `Problem `1 -> `problem1.rs`, `Problem 2` -> `problem2.rs` etc.

The compilation is done using Rust's cargo tool.

## Pre-requisites

Ensure that you have the latest stable `Rust` compiler and associated build tools (`Cargo`) installed. Verify that you are able to install external crates. The code here has been built and tested with Rust version 1.34.2 .

## Building code

### Using Makefile

Just run `make` on the repo.

    $ make

Do `make clean` to clean the binary targets.

	$ make clean

### Manual

Run the script `build_cargo.py` using python to create a `Cargo.toml` from the rust source files. Now build the code using,

    $ cargo build

## Run solution

A given solution can be run by using its binary. Example to run solution to `Problem 1`

    $ cargo run --bin problem1

## Principles

1. All the code here is originally written by the author. No code has been copied from any other source.
2. An effort has been maintained to write code with least compiler warnings and generally keeping up with Rust coding principles. However the code is not pedantic Rust.
3. Almost all code is written with performance in mind and keep up with Project Euler principle of running under a minute. 

[1]: http://projecteuler.net


## Copyright

Copyright &copy; 2020 - Anand B Pillai &lt;anandpillai at letterboxes dot org&gt;
