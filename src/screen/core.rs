use minifb::{Key, Window, WindowOptions, Error};

pub const WIDTH: u8 = 64;
pub const HEIGHT: u8 = 32;

#[cfg(test)]
fn construct_window() -> Result<Window, Error> {
    Err(Error::WindowCreate(String::from("Cannot create window")))
}

#[cfg(not(test))]
fn construct_window() -> Result<Window, Error> {
    Window::new(
        "Chip-8 Emulator",
        WIDTH as usize,
        HEIGHT as usize,
        WindowOptions {
            resize: false,
            scale: minifb::Scale::X8,
            ..WindowOptions::default()
        }
    )
}

#[derive(Debug)]
pub struct Screen {
    pub frame: Vec<u32>,
    pub window: Option<Window>,
    pub pixel_on: u32,
    pub pixel_off: u32,
}

impl Screen {
    pub fn new() -> Self {
        let window = match construct_window() {
            Ok(win) => Some(win),
            _ => None,
        };

        Self {
            window: window,
            frame: [0; 64 * 32].to_vec(),
            pixel_on: 0xC6C5B9,
            pixel_off: 0x62929E,
        }
    }

    pub fn dev_pattern(&mut self) {
        for (i, pixel) in self.frame.iter_mut().enumerate() {
            let x = i % WIDTH as usize;
            let y = i / WIDTH as usize;

            *pixel = ((x ^ y) as u32) << 8;
        }
    }

    pub fn default_pattern(&mut self) {
        for (i, pixel) in self.frame.iter_mut().enumerate() {
            if i % 2 == 0 {
                *pixel = self.pixel_on;
            } else {
                *pixel = self.pixel_off;
            }
        }
    }

    pub fn color(&mut self, color: u32) {
        for pixel in self.frame.iter_mut() {
            *pixel = color;
        }
    }

    pub fn is_running(&self) -> bool {
        self.window.as_ref().unwrap().is_open() && 
        !self.window.as_ref().unwrap().is_key_down(Key::Escape)
    }

    pub fn update(&mut self) {
            self.window
                .as_mut()
                .unwrap()
                .update_with_buffer(&self.frame, WIDTH as usize, HEIGHT as usize)
                .unwrap();
    }
}