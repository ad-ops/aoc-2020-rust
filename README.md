# Advent of Code 2020 (Rust)
This year I am making a way to organize my project around solving the challenges (instead of solving them?).

The main goal is to make it easier to add new days and solutions instead of copy-pasting boiler plate code.

### Bonus
Would have been cool if it was possible to just swap the solvers dynamically. WebAssembly modules for example?

## Setup
New puzzles can use the template of `bin/day0.rs` and change the solvers and test cases. Also `input/day%d_(input|test).txt` needs to be added with the input data provided by Advent of Code.

### Test
`cargo test --bin day0`

### Run
`cargo run --bin day0`


## Lessons Learned
### Day 1
- Can break outer loops.