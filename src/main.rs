#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq)]
enum ParseError {
    UnmatchedOpeningParenthesis,
    UnmatchedClosingParenthesis,
    DivisionByZero,
    InvalidCharacter,
    EmptyExpression,
}

use std::fmt;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            ParseError::UnmatchedOpeningParenthesis => "Unmatched opening parenthesis",
            ParseError::UnmatchedClosingParenthesis => "Unmatched closing parenthesis",
            ParseError::DivisionByZero => "Division by zero",
            ParseError::InvalidCharacter => "Invalid character",
            ParseError::EmptyExpression => "Empty expression",
        };
        write!(f, "{}", description)
    }
}

fn parse_operator(c: char) -> Result<Operator, ParseError> {
    match c {
        'a' => Ok(Operator::Add),
        'b' => Ok(Operator::Subtract),
        'c' => Ok(Operator::Multiply),
        'd' => Ok(Operator::Divide),
        _ => Err(ParseError::InvalidCharacter),
    }
}

fn evaluate_expression(expr: &str) -> Result<i32, ParseError> {
    let mut num_stack: Vec<i32> = Vec::new();
    let mut op_stack: Vec<Operator> = Vec::new();
    let mut iter = expr.chars().peekable();

    while let Some(&c) = iter.peek() {
        match c {
            '0'..='9' => {
                let mut num = 0;
                while let Some('0'..='9') = iter.peek() {
                    num = num * 10 + iter.next().unwrap().to_digit(10).unwrap() as i32;
                }
                num_stack.push(num);
            }
            'a' | 'b' | 'c' | 'd' => {
                let op = parse_operator(c)?;
                iter.next();
                op_stack.push(op);
            }
            'e' => {
                iter.next();
                let mut sub_expr = String::new();
                let mut open_paren = 1;

                while let Some(&c) = iter.peek() {
                    match c {
                        'e' => open_paren += 1,
                        'f' => open_paren -= 1,
                        _ => {}
                    }

                    if open_paren == 0 {
                        iter.next();
                        break;
                    }

                    if iter.peek().is_none() {
                        return Err(ParseError::UnmatchedOpeningParenthesis);
                    }

                    sub_expr.push(iter.next().unwrap());
                }

                if open_paren != 0 {
                    return Err(ParseError::UnmatchedOpeningParenthesis);
                }

                let sub_result = evaluate_expression(&sub_expr)?;
                num_stack.push(sub_result);
            }
            'f' => {
                return Err(ParseError::UnmatchedClosingParenthesis);
            }
            _ => return Err(ParseError::InvalidCharacter),
        }
    }

    if num_stack.is_empty() {
        return Err(ParseError::EmptyExpression);
    }

    let mut result = num_stack[0];
    for i in 1..num_stack.len() {
        match op_stack[i - 1] {
            Operator::Add => result += num_stack[i],
            Operator::Subtract => result -= num_stack[i],
            Operator::Multiply => result *= num_stack[i],
            Operator::Divide => {
                if num_stack[i] == 0 {
                    return Err(ParseError::DivisionByZero);
                }
                result /= num_stack[i];
            }
        }
    }
    Ok(result)
}

use std::io::{self, Write};
fn main() {
    loop {
        // Print a prompt and flush it to stdout
        print!("arithmetic-parser> ");
        io::stdout().flush().unwrap();

        // Read a line of input from stdin
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading line");
            continue;
        }

        // Remove the trailing newline character
        let expression = input.trim();

        // Exit the REPL if the user types "exit"
        if expression == "exit" {
            break;
        }

        // Evaluate the expression
        match evaluate_expression(expression) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => println!("Error: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{evaluate_expression, ParseError};

    #[test]
    fn test_evaluate_expression() {
        let test_cases = vec![
            ("3a2c4", Ok(20)),
            ("32a2d2", Ok(17)),
            ("500a10b66c32", Ok(14208)),
            ("3ae4c66fb32", Ok(235)),
            ("3c4d2aee2a4c41fc4f", Ok(990)),
            ("3ae4c2a2fb32", Ok(-19)),
            ("", Err(ParseError::EmptyExpression)),
            ("g", Err(ParseError::InvalidCharacter)),
            ("0", Ok(0)),
        ];

        for (input, expected) in test_cases {
            let result = evaluate_expression(input);
            assert_eq!(result, expected, "Failed on input: {}", input);
        }
    }

    #[test]
    fn test_unmatched_parenthesis() {
        let result = evaluate_expression("3ae4c66fb32f");
        assert_eq!(result, Err(ParseError::UnmatchedClosingParenthesis));
    }

    #[test]
    fn test_unmatched_opening_parenthesis() {
        let result = evaluate_expression("e1");
        assert_eq!(result, Err(ParseError::UnmatchedOpeningParenthesis));
    }

    #[test]
    fn test_division_by_zero() {
        let result = evaluate_expression("3d0");
        assert_eq!(result, Err(ParseError::DivisionByZero));
    }
}


