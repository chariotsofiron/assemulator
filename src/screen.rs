use core::time::Duration;

use crate::color::Color;
use minifb::{Window, WindowOptions};

/// Width and height of a cell in pixels.
const SIZE_PX: usize = 10; // width/height of cell
/// Width of the screen in pixels.
const WIDTH: usize = 64 * SIZE_PX;
/// Height of the screen in pixels.
const HEIGHT: usize = 64 * SIZE_PX;

/// Frames per second.
const FPS: u64 = 30;
/// The time between frames.
const MILLIS_HZ: std::time::Duration = std::time::Duration::from_millis(1000 / FPS);

/// Double-buffered screen.
/// We write to the buffer first, then copy to the screen.
/// We can read what's currently written to the screen but not the buffer.
pub struct Screen {
    /// The buffer
    buffer: [Color; WIDTH * HEIGHT],
    /// The screen
    screen: [Color; WIDTH * HEIGHT],
    /// The window
    window: Window,

    /// Time of last draw, used for managing FPS.
    last_draw: std::time::Instant,
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            buffer: [Color::Black; WIDTH * HEIGHT],
            screen: [Color::Black; WIDTH * HEIGHT],
            window: Window::new("Assemulator", WIDTH, HEIGHT, WindowOptions::default()).unwrap(),
            last_draw: std::time::Instant::now(),
        }
    }
}

impl Screen {
    /// Writes a pixel with the given color to the frame buffer.
    pub fn plot(&mut self, x: u8, y: u8, color: Color) {
        let pos_x = usize::from(x);
        let pos_y = usize::from(y);
        // update buffer with new pixel according to size_px
        for i in 0..SIZE_PX {
            for j in 0..SIZE_PX {
                self.buffer[(pos_x * SIZE_PX + i) + (pos_y * SIZE_PX + j) * WIDTH] = color;
            }
        }
    }

    /// Reads a pixel from the screen.
    pub fn read_pixel(&self, x: u8, y: u8) -> Color {
        let pos_x = usize::from(x);
        let pos_y = usize::from(y);
        self.screen[(pos_x * SIZE_PX) + (pos_y * SIZE_PX) * WIDTH]
    }

    /// Waits for the next frame to synchonize drawing.
    fn wait_for_frame(&mut self) {
        let now = std::time::Instant::now();
        let elapsed = now - self.last_draw;
        if elapsed < MILLIS_HZ {
            std::thread::sleep(MILLIS_HZ - elapsed);
        }
        self.last_draw = std::time::Instant::now();
    }

    /// Returns the state of the buttons.
    pub fn buttons(&self) -> u8 {
        let mut buttons = 0;
        for key in self.window.get_keys() {
            match key {
                minifb::Key::Up => buttons |= 1,
                minifb::Key::Down => buttons |= 2,
                minifb::Key::Left => buttons |= 4,
                minifb::Key::Right => buttons |= 8,
                minifb::Key::Z => buttons |= 16,
                minifb::Key::X => buttons |= 32,
                _ => {}
            }
        }
        buttons
    }

    /// Returns the state of the buttons, but only the first read.
    pub fn buttonsp(&self) -> u8 {
        let mut buttons = 0;
        for key in self.window.get_keys_pressed(minifb::KeyRepeat::No) {
            match key {
                minifb::Key::Up => buttons |= 1,
                minifb::Key::Down => buttons |= 2,
                minifb::Key::Left => buttons |= 4,
                minifb::Key::Right => buttons |= 8,
                minifb::Key::Z => buttons |= 16,
                minifb::Key::X => buttons |= 32,
                _ => {}
            }
        }
        buttons
    }

    /// Draws the buffer to the screen.
    pub fn draw(&mut self) {
        self.screen = self.buffer;
        self.wait_for_frame();
        let (width, height) = self.window.get_size();

        let buffer = self
            .buffer
            .iter()
            .map(|&x| {
                let (r, g, b) = x.to_rgb();
                u32::from_be_bytes([0, r, g, b])
            })
            .collect::<Vec<_>>();
        self.window
            .update_with_buffer(&buffer, width, height)
            .unwrap();
    }

    /// Draws the buffer to the screen and clear the buffer.
    pub fn flip(&mut self) {
        self.draw();
        self.buffer = [Color::Black; WIDTH * HEIGHT];
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        // keep the screen open if anything was written to it
        while self.window.is_open() && self.buffer.iter().any(|&x| x != Color::Black) {
            self.draw();
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}
