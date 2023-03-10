use minifb::{Window, WindowOptions};

use crate::word::UInt;

const SIZE_PX: usize = 10; // width/height of cell
const WIDTH: usize = 64 * SIZE_PX;
const HEIGHT: usize = 64 * SIZE_PX;

const FPS: u64 = 30;
const MILLIS_HZ: std::time::Duration = std::time::Duration::from_millis(1000 / FPS);

/// Colors
pub const WHITE: (u8, u8, u8) = (255, 241, 232);

pub struct Screen {
    buffer: [u32; WIDTH * HEIGHT],
    window: Window,

    /// Time of last draw, used for managing FPS.
    last_draw: std::time::Instant,
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            buffer: [0; WIDTH * HEIGHT],
            window: Window::new("Assemulator", WIDTH, HEIGHT, WindowOptions::default()).unwrap(),
            last_draw: std::time::Instant::now() - MILLIS_HZ,
        }
    }
}

impl Screen {
    pub fn plot(&mut self, x: u8, y: u8, rgb: (u8, u8, u8)) {
        let x = usize::from(x);
        let y = usize::from(y);
        let color = u32::from_be_bytes([0, rgb.0, rgb.1, rgb.2]);

        // update buffer with new pixel according to size_px
        for i in 0..SIZE_PX {
            for j in 0..SIZE_PX {
                self.buffer[(x * SIZE_PX + i) + (y * SIZE_PX + j) * WIDTH] = color;
            }
        }
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
    pub fn buttons<T: UInt>(&self) -> T {
        let mut buttons = T::from(0);
        for key in self.window.get_keys() {
            match key {
                minifb::Key::Up => buttons = buttons | T::from(1),
                minifb::Key::Down => buttons = buttons | T::from(2),
                minifb::Key::Left => buttons = buttons | T::from(4),
                minifb::Key::Right => buttons = buttons | T::from(8),
                minifb::Key::Z => buttons = buttons | T::from(16),
                minifb::Key::X => buttons = buttons | T::from(32),
                _ => {}
            }
        }
        buttons
    }

    /// Returns the state of the buttons, but only the first read.
    pub fn buttonsp<T: UInt>(&self) -> T {
        let mut buttons = T::from(0);
        for key in self.window.get_keys_pressed(minifb::KeyRepeat::No) {
            match key {
                minifb::Key::Up => buttons = buttons | T::from(1),
                minifb::Key::Down => buttons = buttons | T::from(2),
                minifb::Key::Left => buttons = buttons | T::from(4),
                minifb::Key::Right => buttons = buttons | T::from(8),
                minifb::Key::Z => buttons = buttons | T::from(16),
                minifb::Key::X => buttons = buttons | T::from(32),
                _ => {}
            }
        }
        buttons
    }

    pub fn draw(&mut self) {
        self.wait_for_frame();
        let (width, height) = self.window.get_size();
        self.window
            .update_with_buffer(&self.buffer, width, height)
            .unwrap();
    }

    pub fn flip(&mut self) {
        self.draw();
        self.buffer = [0; WIDTH * HEIGHT];
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        // keep the screen open if anything was written to it
        while self.window.is_open() && self.buffer.iter().any(|&x| x != 0) {
            self.draw();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
