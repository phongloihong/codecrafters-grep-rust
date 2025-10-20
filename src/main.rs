use std::env;
use std::io;
use std::process;

enum MatchError {
    InvalidPattern(String),
}

fn match_pattern(input_line: &str, pattern: &str) -> Result<bool, MatchError> {
    match pattern {
        p if p.len() == 1 => Ok(input_line.contains(p)),
        "\\d" => Ok(input_line.chars().any(|c| c.is_ascii_digit())),
        "\\w" => Ok(input_line.chars().any(|c| c.is_alphanumeric() || c == '_')),
        p if p.starts_with('[') && p.ends_with(']') => {
            let match_str = &p[1..(p.len() - 1)];

            Ok(input_line.chars().any(|c| match_str.contains(c)))
        }
        _ => Err(MatchError::InvalidPattern(pattern.to_string())),
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    match match_pattern(&input_line, &pattern) {
        Ok(true) => process::exit(0),
        Ok(false) => process::exit(1),
        Err(MatchError::InvalidPattern(p)) => {
            eprint!("Unhandled pattern: {}", p);
            process::exit(1);
        }
    }
}
