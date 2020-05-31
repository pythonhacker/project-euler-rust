## About

The repo contains solutions to [Project Euler][1] in Rust.

## Source files

Each source file maps to a given Project Euler problem by index. So `Problem `1 -> `problem1.rs`, `Problem 2` -> `problem2.rs` etc.

The compilation is done using Rust's cargo tool.

## Create `Cargo.toml`

Run the script `build_cargo.py` using python to create a `Cargo.toml` from the rust source files.

## Build code

    $ cargo build

## Run solution

A given solution can be run by using its binary. Example to run solution to `Problem 1`

    $ cargo run --bin problem1


[1]: http://projecteuler.net


## Copyright

Copyright &copy; 2020 - Anand B Pillai &lt;anandpillai at letterboxes dot org&gt; See LICENSE for more details.
