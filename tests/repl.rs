use std::io::{BufReader, Cursor};

use testing_demo::repl;

#[test]
fn test_repl() {
    let mut input = BufReader::new("\
        abc\n\
        -5\n\
        3.14\n\
        0\n\
        1\n\
        2\n\
        3\n\
        5\n\
        10\n\
        50\n\
        quit\n\
    ".as_bytes());
    let mut output = Cursor::new(Vec::new());

    repl(&mut input, &mut output).unwrap();

    let output = String::from_utf8(output.into_inner()).unwrap();
    assert_eq!(output, "\
        Enter a number: \
        Input must be a non-negative integer\n\
        Enter a number: \
        Input must be a non-negative integer\n\
        Enter a number: \
        Input must be a non-negative integer\n\
        Enter a number: \
        0! = 1\n\
        Enter a number: \
        1! = 1\n\
        Enter a number: \
        2! = 2\n\
        Enter a number: \
        3! = 6\n\
        Enter a number: \
        5! = 120\n\
        Enter a number: \
        10! = 3628800\n\
        Enter a number: \
        Integer overflow when computing factorial\n\
        Enter a number: \
        Goodbye\n\
    ");
}
