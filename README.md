# Advent of Code 2022
My Rust solutions to [Advent of Code 2022](https://adventofcode.com/2022).

## Usage
To build: `cargo build --release`  
To run a solution: `aoc-2022 <day> <part>` e.g. `aoc-2022 day1 part1`

Puzzle input is accepted through stdin, and result is written to stdout.

Add `--time` as the first argument to have the calculation time outputted in μs.

### Currently completed puzzles
A star (*) next to a day means that there is a revised solution present, which was not
what I used to first solve the puzzle, but what I deem a better solution/implementation
which I came up with after initially solving the puzzle. Revised solutions are accessible
by appending `_revised` to the day, e.g., `aoc-2022 day1_revised part1`. All non-revised
solutions are the (almost) exact original code (some adjustments may have been made to the
organization of the parts, which consequently minorly affected the code for the parts
themselves), including `cargo clippy` violations, which will not be fixed outside of
revised versions; as a result, there may be `#[allow(…)]` attributes applied to some of
the original solutions to disable the violated lints.

* [x] [Day 1](src/days/day1.rs)
  * [x] Part 1
  * [x] Part 2
* [x] [Day 2](src/days/day2.rs)[*](src/days/day2_revised.rs)
  * [x] Part 1
  * [x] Part 2
* [x] [Day 3](src/days/day3.rs)
  * [x] Part 1
  * [x] Part 2
* [x] [Day 4](src/days/day4.rs)
  * [x] Part 1
  * [x] Part 2
* [x] [Day 5](src/days/day5.rs)
  * [x] Part 1
  * [x] Part 2
* [x] [Day 6](src/days/day6.rs)
  * [x] Part 1
  * [x] Part 2
* [x] [Day 7](src/days/day7.rs)
  * [x] Part 1
  * [x] Part 2
* [ ] Day 8
  * [ ] Part 1
  * [ ] Part 2
* [ ] Day 9
  * [ ] Part 1
  * [ ] Part 2
* [ ] Day 10
  * [ ] Part 1
  * [ ] Part 2
* [ ] Day 11
  * [ ] Part 1
  * [ ] Part 2
* [ ] Day 12
  * [ ] Part 1
  * [ ] Part 2
* [ ] Day 13
  * [ ] Part 1
  * [ ] Part 2
* [ ] Day 14
  * [ ] Part 1
  * [ ] Part 2
* [ ] Day 15
  * [ ] Part 1
  * [ ] Part 2
* [ ] Day 16
  * [ ] Part 1
  * [ ] Part 2
* [ ] Day 17
  * [ ] Part 1
  * [ ] Part 2
* [ ] Day 18
  * [ ] Part 1
  * [ ] Part 2
* [ ] Day 19
  * [ ] Part 1
  * [ ] Part 2
* [ ] Day 20
  * [ ] Part 1
  * [ ] Part 2
* [ ] Day 21
  * [ ] Part 1
  * [ ] Part 2
* [ ] Day 22
  * [ ] Part 1
  * [ ] Part 2
* [ ] Day 23
  * [ ] Part 1
  * [ ] Part 2
* [ ] Day 24
  * [ ] Part 1
  * [ ] Part 2
* [ ] Day 25
  * [ ] Part 1
  * [ ] Part 2
