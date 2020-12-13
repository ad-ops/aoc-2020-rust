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
- Can label and break outer loops.

### Day 2
- Macros to reduce code.
- `parse_display` crate for parsing a string into a struct.

### Day 3
- Should have used a for-loop to make it easier to swap stepping numbers.

### Day 4
- `inspect`-method to print-debug inside iterator.

### Day 5
- Binary numbers in Rust.

### Day 6
- `HashMap` / `HashSet` and internal iterators.

### Day 7

### Day 8
- Error transformation and handling.
- FromStr-trait and parse-method.
- Ugly brute force sometimes works fine (in ms span for this task).

### Day 9
- Need more flexible tests / input...
- Need to learn more of how move and pointers work...
- Need to be better at looping or iterating...

### Day 10
- peekable iterators and loop to check next value.

### Day 11
- Very ugly and repetitive code. Need to refactor...

### Day 12
- Fun how much high school math that has been forgotten:-).

### Day 13
- Took some time before realizing the stepping could be simplyfied.