use std::io::{stdin, stdout};

use testing_demo::repl;

fn main() {
    repl(&mut stdin().lock(), &mut stdout().lock())
        .expect("an I/O error occurred");
}
