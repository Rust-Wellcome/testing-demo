use std::io::{Write, BufRead};

use crate::factorial;

/// Runs a [Read-Eval-Print-Loop][REPL] which prompts the user for a number and
/// then prints the factorial of that number.
/// 
/// # Errors
/// Returns an error if an I/O operation fails.
/// 
/// [REPL]: https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop
pub fn repl(input: &mut impl BufRead, output: &mut impl Write) -> std::io::Result<()> {
    let mut buffer = String::new();
    loop {
        write!(output, "Enter a number: ")?;
        output.flush()?;
        
        buffer.clear();
        input.read_line(&mut buffer)?;
        
        let input = buffer.trim();
        if matches!(input.to_lowercase().as_str(), "quit" | "q") {
            break;
        }

        let Ok(number) = input.parse() else {
            writeln!(output, "Input must be a non-negative integer")?;
            continue;
        };

        let Some(answer) = factorial(number) else {
            writeln!(output, "Integer overflow when computing factorial")?;
            continue;
        };

        writeln!(output, "{number}! = {answer}")?;
    }
    writeln!(output, "Goodbye")?;
    Ok(())
}
