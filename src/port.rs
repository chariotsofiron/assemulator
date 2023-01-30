use std::collections::VecDeque;
use strum_macros::EnumIter;

use crate::{
    screen::Screen,
    screen::WHITE,
    util::{input, read_int},
    word::UInt,
};

/// The port's state.
/// T is the word size of the CPU, should be one of u8, u16, u32, u64, u128
#[derive(Default)]
pub struct Port<T: UInt> {
    /// The buffer of chars read from stdin.
    chars: VecDeque<u8>,

    /// Graphics
    x_coord: T,
    y_coord: T,
    screen: Screen,
}

impl<T> Port<T>
where
    T: UInt,
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: std::fmt::Debug,
    <T as TryFrom<u64>>::Error: std::fmt::Debug,
{
    pub fn read_port(&mut self, port: PortAddress) -> T {
        match port {
            PortAddress::Char => {
                // if there are chars in the buffer, read from it
                // otherwise read from stdin
                if self.chars.is_empty() {
                    self.chars.extend(input("> ").unwrap().as_bytes());
                }
                match self.chars.pop_front() {
                    Some(c) => T::from(c),
                    None => T::from(0),
                }
            }
            PortAddress::Ticker => read_int::<T>(),
            PortAddress::Random => {
                let mask = (1 << std::mem::size_of::<T>()) - 1;
                T::try_from(rand::random::<u64>() & mask).unwrap()
            }
            PortAddress::XPos => self.x_coord,
            PortAddress::YPos => self.y_coord,
            PortAddress::Flip | PortAddress::Draw => T::from(0),
            PortAddress::Buttons => self.screen.buttons::<T>(),
            PortAddress::ButtonsP => self.screen.buttonsp::<T>(),
        }
    }

    pub fn write_port(&mut self, port: PortAddress, value: T) {
        match port {
            PortAddress::Char => {
                print!("{}", char::from(u8::try_from(value).unwrap()));
            }
            PortAddress::Ticker => println!("{}", value),
            PortAddress::XPos => self.x_coord = value,
            PortAddress::YPos => {
                self.y_coord = value;
                self.screen.plot(
                    u8::try_from(self.x_coord).unwrap(),
                    u8::try_from(self.y_coord).unwrap(),
                    WHITE,
                );
            }
            PortAddress::Flip => self.screen.flip(),
            PortAddress::Draw => self.screen.draw(),
            PortAddress::Random | PortAddress::Buttons | PortAddress::ButtonsP => {}
        }
    }
}

/// The different ports.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum PortAddress {
    /// Print character to screen, read char from stdin
    Char,
    /// Print unsigned byte to screen, read byte from stdin
    Ticker,
    Random,

    /// Graphics
    XPos,
    YPos,
    Flip,
    Draw,
    Buttons,
    ButtonsP,
}

impl TryFrom<usize> for PortAddress {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Char),
            1 => Ok(Self::Ticker),
            2 => Ok(Self::Random),
            8 => Ok(Self::XPos),
            9 => Ok(Self::YPos),
            10 => Ok(Self::Flip),
            11 => Ok(Self::Draw),
            12 => Ok(Self::Buttons),
            13 => Ok(Self::ButtonsP),

            _ => Err(format!("Invalid port address: {}", value)),
        }
    }
}

impl From<PortAddress> for usize {
    fn from(port: PortAddress) -> Self {
        match port {
            PortAddress::Char => 0,
            PortAddress::Ticker => 1,
            PortAddress::Random => 2,
            PortAddress::XPos => 8,
            PortAddress::YPos => 9,
            PortAddress::Flip => 10,
            PortAddress::Draw => 11,
            PortAddress::Buttons => 12,
            PortAddress::ButtonsP => 13,
        }
    }
}
