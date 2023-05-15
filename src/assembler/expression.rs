use std::str::Chars;
use std::iter::Peekable;

fn main() {
    let input = "3.6 + 4 * (2 - 3) / 2.4";
    let mut parser = Parser::new(input);
    let result = parser.parse_expression(0);
    match result {
        Ok(value) => println!("Result: {:?}", value),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

pub struct Parser<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            input: input.chars().peekable(),
        }
    }

    pub fn parse_expression(&mut self, precedence: i32) -> Result<f64, String> {
        self.skip_whitespace();
        let mut left = self.parse_factor()?;

        while let Some(&c) = self.input.peek() {
            let op_precedence = match c {
                '+' | '-' => 1,
                '*' | '/' => 2,
                _ => break,
            };

            if op_precedence <= precedence {
                break;
            }

            self.input.next();
            let right = self.parse_expression(op_precedence)?;
            left = match c {
                '+' => left + right,
                '-' => left - right,
                '*' => left * right,
                '/' => left / right,
                _ => unreachable!(),
            };
            self.skip_whitespace();
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<f64, String> {
        match self.input.peek() {
            Some(&c) if c == '(' => {
                self.input.next();
                let result = self.parse_expression(0);
                match self.input.next() {
                    Some(')') => result,
                    Some(_) | None => Err("Mismatched parenthesis".to_owned()),
                }
            }
            Some(_) => {
                let mut value = String::new();
                while let Some(&c) = self.input.peek() {
                    if c.is_numeric() || c == '.' {
                        value.push(c);
                        self.input.next();
                    } else {
                        break;
                    }
                }
                value
                    .parse::<f64>()
                    .map_err(|e| format!("Failed to parse number: {:?}", e))
            }
            None => Err("Unexpected end of input".to_owned()),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }
}