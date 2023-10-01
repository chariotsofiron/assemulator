use std::collections::HashMap;

use crate::util;

// these should be sorted by length for maximal munch
const OPERATORS: [(&str, usize); 18] = [
    ("||", 0),
    ("&&", 1),
    ("==", 2),
    ("!=", 2),
    ("<=", 2),
    (">=", 2),
    ("<<", 9),
    (">>", 9),
    ("<", 2),
    (">", 2),
    ("|", 3),
    ("^", 4),
    ("&", 5),
    ("+", 10),
    ("-", 10),
    ("*", 20),
    ("/", 20),
    ("%", 20),
];

pub fn parse<'a>(
    input: &str,
    symbols: &'a HashMap<String, u64>,
    resolve_labels: bool,
) -> Result<u64, String> {
    let mut parser = Parser::new(input, symbols, resolve_labels);
    let result = parser.parse_expression(0);
    if parser.i < parser.input.len() {
        Err(format!(
            "Unexpected input at position {}: {}",
            parser.i, &parser.input[parser.i..]
        ))
    } else {
        result
    }
}

pub struct Parser<'a> {
    symbols: &'a HashMap<String, u64>,
    resolve_labels: bool,
    input: String,
    i: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &str, symbols: &'a HashMap<String, u64>, resolve_labels: bool) -> Self {
        Parser {
            symbols,
            resolve_labels,
            input: input.replace(" ", ""),
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
        let mut left = self.parse_factor()?;
        while let Some((op, prec)) = OPERATORS
            .iter()
            .find_map(|(x, y)| self.input[self.i..].starts_with(x).then(|| (*x, *y)))
        {
            if prec < precedence {
                break;
            }
            self.accept(op);
            let right = self.parse_expression(prec + 1)?;

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
                "&&" => u64::from(left != 0 && right != 0),
                "||" => u64::from(left != 0 || right != 0),
                _ => unreachable!(),
            };
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<u64, String> {
        if self.accept("(").is_some() {
            let result = self.parse_expression(0)?;
            self.accept(")").ok_or("Expected closing parentheses")?;
            Ok(result)
        } else if self.accept("-").is_some() {
            self.parse_factor().map(|x| (!x).wrapping_add(1)) // negate by 2s complement
        } else if self.input[self.i..].starts_with(char::is_alphabetic) {
            let start = self.i;
            while self.input[self.i..].starts_with(char::is_alphanumeric)
                || self.input[self.i..].starts_with('_')
            {
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
            while self.input[self.i..].starts_with(char::is_alphanumeric)
                || self.input[self.i..].starts_with('_')
            {
                self.i += 1;
            }
            let num = &self.input[start..self.i];
            util::parse_int(num)
        } else if self.i >= self.input.len() {
            Err(format!("Unexpected end of input"))
        } else {
            Err(format!(
                "Expected number or identifier {}",
                &self.input[self.i..]
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atom() {
        let input = "1";
        let map = HashMap::new();
        let result = parse(&input, &map, false);
        assert_eq!(result, Ok(1));
    }

    #[test]
    fn test_parse() {
        let input = "1+2*3";
        let map = HashMap::new();
        let result = parse(&input, &map, false);
        assert_eq!(result, Ok(7));
    }

    #[test]
    fn test2() {
        let input = "10 >> 1 == 0";
        let map = HashMap::new();
        let result = parse(&input, &map, false);
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test3() {
        let input = "12 >> 6 != 0 && (12 >> 6) & 1023 != 1023";
        let map = HashMap::new();
        let result = parse(&input, &map, false);
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test4() {
        let input = "23#";
        let map = HashMap::new();
        let result = parse(&input, &map, false);
        assert_eq!(result, Err("Unexpected input at position 2: #".to_string()));
    }
}
