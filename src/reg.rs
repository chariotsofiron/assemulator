//! Register file.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Register<const N: usize>(pub usize);

impl<const N: usize> TryFrom<&str> for Register<N> {
    type Error = String;

    fn try_from(reg: &str) -> Result<Self, Self::Error> {
        let mut chars = reg.chars();

        if chars.next() != Some('r') {
            return Err(format!("Invalid register: {reg}"));
        }

        let value = chars
            .as_str()
            .parse::<usize>()
            .map_err(|_| format!("Invalid register: {reg}"))?;

        if value >= N {
            return Err(format!("Invalid register: {reg}"));
        }
        Ok(Register(value))
    }
}
