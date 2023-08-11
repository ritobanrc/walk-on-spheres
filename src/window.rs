use image::{save_buffer, ColorType};
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use ndarray::Array2;
use std::convert::AsRef;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(pub [u8; 4]);

impl From<Color> for u32 {
    fn from(color: Color) -> u32 {
        let color = color.0;
        (color[0] as u32) << 16 | (color[1] as u32) << 8 | (color[2] as u32)
    }
}

pub struct RenderWindow<'a> {
    title: &'a str,
    options: WindowOptions,
    width: usize,
    height: usize,
    fps: u64,
}

impl<'a> RenderWindow<'a> {
    pub fn new(
        title: &'a str,
        options: WindowOptions,
        width: usize,
        height: usize,
    ) -> RenderWindow {
        RenderWindow {
            title,
            options,
            width,
            height,
            fps: 60,
        }
    }

    pub fn display(&self, mut render: impl FnMut() -> Array2<Color>) {
        let mut window = Window::new(self.title, self.width, self.height, self.options)
            .unwrap_or_else(|e| {
                panic!("Window creation failed -- {}", e);
            });

        window.limit_update_rate(Some(std::time::Duration::from_millis(1000 / self.fps)));

        while window.is_open() && !window.is_key_down(Key::Escape) {
            let frame = render();
            let slice = frame.as_slice().expect("failed to get slice from Array2");

            let buffer: Vec<u32> = slice.iter().map(|c| u32::from(*c)).collect();

            if window.is_key_pressed(Key::F3, KeyRepeat::No) {
                let filename = format!("./{}.png", self.title);
                println!("Saving image to {}", filename);
                save_image(&slice, filename, self.width, self.height)
            }
            window
                .update_with_buffer(&buffer, self.width, self.height)
                .unwrap();
        }
    }
}

pub fn save_image<P>(render: &[Color], path: P, width: usize, height: usize)
where
    P: AsRef<Path>,
{
    let new_buf: Vec<u8> = render.iter().flat_map(|&x| x.0).collect();
    save_buffer(
        path,
        &new_buf,
        width as u32,
        height as u32,
        ColorType::Rgba8,
    )
    .expect("Failed to save");
}
