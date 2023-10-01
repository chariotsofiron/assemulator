//! Register file.

use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Register<const N: usize>(pub usize);

impl<const N: usize> FromStr for Register<N> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        if chars.next() != Some('r') {
            return Err(format!("Invalid register: {s}"));
        }

        let value = chars
            .as_str()
            .parse::<usize>()
            .map_err(|_| format!("Invalid register: {s}"))?;

        if value >= N {
            return Err(format!("Invalid register: {s}"));
        }
        Ok(Register(value))
    }
}
