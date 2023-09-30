use std::collections::HashMap;

fn main() {
    let input = "1+foo&";
    let map = HashMap::from([("foo".to_string(), 3)]);
    let mut parser = Parser::new(input, &map, true);
    let result = parser.parse_expression(0);
    match result {
        Ok(value) => println!("Result: {:?}", value),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

pub fn parse<'a>(
    input: &'a str,
    symbols: &'a HashMap<String, u64>,
    resolve_labels: bool,
) -> Result<u64, String> {
    Parser::new(input, symbols, resolve_labels).parse_expression(0)
}

pub struct Parser<'a> {
    symbols: &'a HashMap<String, u64>,
    resolve_labels: bool,
    input: &'a str,
    i: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str, symbols: &'a HashMap<String, u64>, resolve_labels: bool) -> Self {
        Parser {
            symbols,
            resolve_labels,
            input,
            i: 0,
        }
    }

    fn accept<'b>(&mut self, tok: &'b str) -> Option<&'b str> {
        self.input[self.i..].starts_with(tok).then(|| {
            self.i += tok.len();
            tok
        })
    }

    pub fn parse_expression(&mut self, precedence: usize) -> Result<u64, String> {
        let operators = [
            ("==", 2),
            ("!=", 2),
            ("<=", 2),
            (">=", 2),
            ("<", 2),
            (">", 2),
            ("|", 3),
            ("^", 4),
            ("&", 5),
            ("<<", 6),
            (">>", 6),
            // addition
            ("+", 10),
            ("-", 10),
            // products
            ("*", 20),
            ("/", 20),
            ("%", 20),
        ];

        self.skip_whitespace();
        let mut left = self.parse_factor()?;
        self.skip_whitespace();

        while let Some((op, prec)) = operators
            .iter()
            .find_map(|(x, y)| self.input[self.i..].starts_with(x).then(|| (*x, *y)))
        {
            if prec <= precedence {
                break;
            }
            self.accept(op).unwrap();
            self.skip_whitespace();
            let right = self.parse_expression(prec)?;
            self.skip_whitespace();
            left = match op {
                "+" => left.wrapping_add(right),
                "-" => left.wrapping_sub(right),
                "*" => left.wrapping_mul(right),
                "/" => left.wrapping_div(right),
                "%" => left.wrapping_rem(right),
                "&" => left & right,
                "|" => left | right,
                "^" => left ^ right,
                "<<" => left.wrapping_shl(right as u32),
                ">>" => left.wrapping_shr(right as u32),
                "<" => u64::from(left < right),
                ">" => u64::from(left > right),
                "<=" => u64::from(left <= right),
                ">=" => u64::from(left >= right),
                "==" => u64::from(left == right),
                "!=" => u64::from(left != right),
                _ => unreachable!(),
            };
        }
        if self.i != self.input.len() {
            return Err(format!(
                "Didn't consume whole expression: {}",
                &self.input[self.i..]
            ));
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<u64, String> {
        println!("bburk `{}", &self.input[self.i..]);
        if self.accept("(").is_some() {
            let result = self.parse_expression(0)?;
            self.accept(")").ok_or("Expected closing parentheses")?;
            Ok(result)
        } else if self.accept("-").is_some() {
            self.parse_factor().map(|x| (!x).wrapping_add(1)) // negate by 2s complement
        } else if self.input[self.i..].starts_with(char::is_alphabetic) {
            let start = self.i;
            while self.input[self.i..].starts_with(char::is_alphanumeric) {
                self.i += 1;
            }
            let ident = &self.input[start..self.i];
            if self.resolve_labels {
                self.symbols
                    .get(ident)
                    .cloned()
                    .ok_or(format!("Undefined identifier {ident}"))
            } else {
                Ok(0)
            }
        } else if self.input[self.i..].starts_with(char::is_numeric) {
            let start = self.i;
            while self.input[self.i..].starts_with(char::is_numeric) {
                self.i += 1;
            }
            let num = &self.input[start..self.i];
            u64::from_str_radix(num, 10).map_err(|e| e.to_string())
        } else if self.i >= self.input.len() {
            Err(format!("Unexpected end of input"))
        } else {
            Err(format!("Expected number or identifier"))
        }
    }

    fn skip_whitespace(&mut self) {
        while self.input[self.i..].starts_with(char::is_whitespace) {
            self.i += 1;
        }
    }
}
