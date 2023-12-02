# Advent Of Rust 2023

This repo tries to implement all the puzzles from the [Advent of code 2023](https://adventofcode.com/). Likely, I am going to quit halfway through though. Christmas is a busy time.

Each branch solves one puzzle and has a second branch for the second half of each puzzle. All branches are named `puzzle_<no>_<1/2>` where the `<no>` is the puzzle number and `<1|2>` tells which half of the puzzle is solved by this branch.

I am reusing lots of stuff from my attempt in 2021 - mainly the text parsing because that's just boilerplate.

## Building

Of course, an installed version of rust is needed. With cargo installed, all puzzles should build using
```bash
cargo build     # to build
cargo run       # to run
```

The inputs and outputs of each puzzle are stored in `input.txt` and `output.txt`.

## Puzzle description

Here, the puzzle description is copied over from the advent of code page for each puzzle.
