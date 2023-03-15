# testing-demo

A demo of the different ways to write tests in Rust.

## Repo overview

The program is a [Read-Eval-Print-Loop][REPL] which prompts the user to enter a number and then prints the [factorial] of their number.

Run the program with `cargo run`. Run the tests with `cargo test`.

[REPL]: https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop
[factorial]: https://en.wikipedia.org/wiki/Factorial

## Unit tests

The `factorial` function has a number of unit tests defined in the `tests` module in `src/factorial.rs`. This module has the `#[cfg(test)]`
attribute, which means it will only be included in compilation when compiling to run tests; it won't bloat the actual built binary.

## Doc tests

The `factorial` function also has doctests in its doc comment. In a real program, these would probably be sufficient,
making the unit tests redundant. For the sake of the example, both the unit and doc tests are left in.

## Integration test

There is also an integration test in `tests/repl.rs`. As is often the case, writing the
integration test required thinking carefully about how to structure the main program. Here, I've made the `repl` function
generic over its input and output – they can be any types implementing `BufRead` and `Write` respectively. This allows the
main program to pass in locks on stdin and stdout, while the integration test can pass in a reader on a static string and a `Cursor`
wrapping a `Vec` of bytes, so the test can be run without live user input.

For a real program, you probably wouldn't want to explicitly test the user-directed output of the program; you should prefer to design your program
such that you can run an integration test on the general logic of the program while still allowing the exact text shown to the user to vary. For the 
sake of simplicity, this design work is omitted in this example.
