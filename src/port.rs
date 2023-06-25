use std::collections::VecDeque;
use strum_macros::EnumIter;
use rand::{distributions::Standard, prelude::Distribution};

use crate::{
    color::Color,
    screen::Screen,
    util::{input, read_int},
};

/// The different ports.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum Port {
    /// Print character to screen, read char from stdin
    Char,
    /// Print unsigned byte to screen, read byte from stdin
    Ticker,
    /// Set seed for random number generator, read random number
    Random,
    /// Write the x position for graphics
    XPos,
    /// Writes a white pixel to (x,y) in the frame buffer.
    /// Reads the color of the pixel at the current position.
    YPos,
    /// Writes the color to (x,y) and sets the color for future writes.
    /// Reads the color of the pixel on the screen at the current position.
    Color,
    /// Write the frame buffer to the display. Clears the frame buffer.
    Flip,
    /// Write the frame buffer to the display.
    Draw,
    Buttons,
    ButtonsP,
}

impl TryFrom<usize> for Port {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Char),
            1 => Ok(Self::Ticker),
            2 => Ok(Self::Random),
            8 => Ok(Self::XPos),
            9 => Ok(Self::YPos),
            10 => Ok(Self::Color),
            11 => Ok(Self::Flip),
            12 => Ok(Self::Draw),
            13 => Ok(Self::Buttons),
            14 => Ok(Self::ButtonsP),

            _ => Err(format!("Invalid port address: {}", value)),
        }
    }
}

impl From<Port> for usize {
    fn from(port: Port) -> Self {
        match port {
            Port::Char => 0,
            Port::Ticker => 1,
            Port::Random => 2,
            Port::XPos => 8,
            Port::YPos => 9,
            Port::Color => 10,
            Port::Flip => 11,
            Port::Draw => 12,
            Port::Buttons => 13,
            Port::ButtonsP => 14,
        }
    }
}

/// The port's state.
/// T is the word size of the CPU, should be one of u8, u16, u32, u64, u128
// We want the state to be generic over the word size of the CPU because some operations
// benefit from the extra data, for example
// - RNG
// - writing a number to the terminal
// - ... what else?
#[derive(Default)]
pub struct PortState<T> {
    /// The buffer of chars read from stdin.
    chars: VecDeque<u8>,

    /// Graphics
    x_coord: T,
    y_coord: T,
    color: Color,
    screen: Screen,
}

impl<T> PortState<T>
where
    T: Copy,
    T: std::fmt::Display,
    T: TryFrom<u64>,
    T: From<u8>,
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: std::fmt::Debug,
    Standard: Distribution<T>,
{
    pub fn read_port(&mut self, port: Port) -> T {
        match port {
            Port::Char => {
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
            Port::Ticker => read_int::<T>(),
            Port::Random => rand::random::<T>(),
            Port::XPos => self.x_coord,
            Port::YPos => self.y_coord,
            Port::Color => T::from(u8::from(self.screen.read_pixel(
                u8::try_from(self.x_coord).unwrap(),
                u8::try_from(self.y_coord).unwrap(),
            ))),
            Port::Flip | Port::Draw => T::from(0),
            Port::Buttons => T::from(self.screen.buttons()),
            Port::ButtonsP => T::from(self.screen.buttonsp()),
        }
    }

    pub fn write_port(&mut self, port: Port, value: T) {
        match port {
            // print char to screen
            Port::Char => {
                print!("{}", char::from(u8::try_from(value).unwrap()));
            }
            // print unsigned integer to screen
            Port::Ticker => println!("{}", value),
            Port::XPos => self.x_coord = value,
            Port::YPos => {
                self.y_coord = value;
                self.screen.plot(
                    u8::try_from(self.x_coord).unwrap(),
                    u8::try_from(self.y_coord).unwrap(),
                    Color::White,
                );
            }
            Port::Color => self.color = Color::from(u8::try_from(value).unwrap()),
            Port::Flip => self.screen.flip(),
            Port::Draw => self.screen.draw(),
            Port::Random | Port::Buttons | Port::ButtonsP => {}
        }
    }
}
