use pixels::{Pixels, SurfaceTexture};
use crate::application::Application;
use winit::window::Window;

#[derive(Debug, PartialEq)]
pub struct Screen<'a> {
    frame: Pixels<'a>,
}

impl Screen<'a> {
    pub fn new(window: &Option<Window>) -> Self {
        let window = SurfaceTexture::new(64, 32, window)
        Self { frame: Pixels::new(64, 32, window).unwrap() }
    }
}