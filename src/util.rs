//! Utility functions.
use core::ops::Neg;
use std::io::Write;

use crate::word::UInt;

/// Masks a signed integer into a field of a given width.
///
/// # Examples
///
/// ```rust
/// use util::mask;
/// assert_eq!(mask(2, 2), Ok(2));
/// assert_eq!(mask(0xff, 8), Ok(0xff));
/// assert_eq!(mask(-2, 4), Ok(0b1110));
/// assert_eq!(mask(5, 0), Ok(0));
/// ```
///  
/// # Errors
///
/// If `value` can't fit inside `n_bits`.
///
pub fn mask<T>(value: u64, n_bits: usize) -> Result<T, String>
where
    T: TryFrom<u64> + Copy,
{
    // Compute up to 2^64 - 1 without overflow
    let mask: u64 = ((1 << (n_bits - 1)) - 1) * 2 + 1;
    T::try_from(value & mask).map_err(|_| "Should not trigger".to_owned())
}

/// Parses a string representing an integer.
/// Works similarly to int(x, 0) in Python.
///
/// # Examples
///
/// ```rust
/// use crate::util::parse_int;
/// assert_eq!(parse_int("0x10"), Some(16));
/// assert_eq!(parse_int("0b1010"), Some(10));
/// assert_eq!(parse_int("0o10"), Some(8));
/// assert_eq!(parse_int("10"), Some(10));
/// assert_eq!(parse_int("-1"), Some(-1));
/// assert_eq!(parse_int("-0b101"), Some(-5));
/// assert_eq!(parse_int("abc"), None);
/// ```
#[allow(clippy::option_if_let_else)]
pub fn parse_int(text: &str) -> Result<u64, String> {
    let value = if let Some(x) = text.strip_prefix('-') {
        Some(parse_int(x).map(|x| (!x) + 1)?)
    } else if let Some(x) = text.strip_prefix("0x") {
        u64::from_str_radix(x, 16).ok()
    } else if let Some(x) = text.strip_prefix("0b") {
        u64::from_str_radix(x, 2).ok()
    } else if let Some(x) = text.strip_prefix("0o") {
        u64::from_str_radix(x, 8).ok()
    } else {
        text.parse().ok()
    };

    value.ok_or_else(|| format!("Invalid integer: {text}"))
}

// /// Converts a floating point value to a fixed point value.
// ///
// /// # Examples
// ///
// /// ```rust
// /// use util::float_to_fixed;
// /// assert_eq!(float_to_fixed(0.5, 8), 0b10000000);
// /// assert_eq!(float_to_fixed(3.1415, 8), 0b11_00100100);
// /// ```
// pub fn _to_fixed(n: f64, fraction: u32) -> u64 {
//     (n * f64::from(1_i32.wrapping_shl(fraction))) as u64
// }

/// Reads an int from user input.
pub fn read_int<T: UInt>() -> T {
    loop {
        match input("> ")
            .map_err(|err| err.to_string())
            .and_then(|x| parse_int(x.trim()))
            .and_then(|x| T::try_from(x).map_err(|_| "Invalid input".to_owned()))
        {
            Ok(x) => break x,
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}

/// Gets input string from user
pub fn input(msg: &str) -> std::io::Result<String> {
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(msg.as_bytes())?;
    stdout.flush()?;
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;

    if buf.ends_with('\n') {
        buf.pop();
        if buf.ends_with('\r') {
            buf.pop();
        }
    }
    Ok(buf)
}
