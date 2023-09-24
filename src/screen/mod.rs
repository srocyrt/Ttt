use std::fmt;
use wasm_bindgen::prelude::*;

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 64;

type Pixel = Option<u8>;

#[wasm_bindgen]
pub struct BitMap {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

#[wasm_bindgen]
impl BitMap {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![None; width * height];
        assert_eq!(width * height, pixels.capacity());
        BitMap {
            width,
            height,
            pixels,
        }
    }

    pub fn fill(&mut self, color: u8) {
        self.pixels.fill(Some(color));
    }

    pub fn white_noise(width: usize, height: usize) -> Self {
        let mut pixels = Vec::with_capacity(width * height);
        for _ in 0..width * height {
            pixels.push(Some(rand::random()));
        }
        BitMap {
            width,
            height,
            pixels,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, p: Pixel) {
        if x > self.width { unreachable!() };
        if y > self.height { unreachable!() };
        self.pixels[self.width * y + x] = p;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Pixel {
        if x > self.width { unreachable!() };
        if y > self.height { unreachable!() };
        self.pixels[self.width * y + x]
    }

}

// ▄█▀
impl fmt::Display for BitMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                write!(
                    f,
                    "\u{001b}[38;5;{}m█\u{001b}[0m",
                    match self.get_pixel(col, row) {
                        Some(x) => x,
                        None => 0,
                    }
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[wasm_bindgen]
pub struct Screen {
    width: usize,
    height: usize,
    background: Option<u8>,
    frame: BitMap,
    buffer: BitMap,
    diff_count: u32,
}

#[wasm_bindgen]
impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        Screen {
            width,
            height,
            background: None,
            frame: BitMap::new(width, height),
            buffer: BitMap::new(width, height),
            diff_count: 0,
        }
    }

    pub fn with_bg(width: usize, height: usize, bg: u8) -> Self {
        let mut frame = BitMap::new(width, height);
        let mut buffer = BitMap::new(width, height);
        frame.fill(bg);
        buffer.fill(bg);
        Screen {
            width,
            height,
            background: Some(bg),
            frame,
            buffer,
            diff_count: 0,
        }
    }

    pub fn paint(&mut self, sprite: BitMap, x: usize, y: usize) {
        let height_bound: usize = if y + sprite.height > self.height { self.height - y } else { sprite.height };
        let width_bound: usize = if x + sprite.width > self.width { self.width - x } else { sprite.width };
        for offset_y in 0..height_bound {
            for offset_x in 0..width_bound {
                self.buffer.set_pixel(x + offset_x, y + offset_y, sprite.get_pixel(offset_x, offset_y))
            }
        }
    }

    pub fn next(&mut self) -> String {
        self.frame.fill(if let Some(x) = self.background { x } else { 0 });
        std::mem::swap(&mut self.frame, &mut self.buffer);
        format!("{}", self.frame)
    }

    pub fn diff(&mut self) -> *const u32 {
        let mut d: Vec<u32> = vec![];
        self.diff_count = 0;
        for i in 0..self.width * self.height {
            match (self.buffer.pixels[i], self.frame.pixels[i]) {
                (Some(x), Some(y)) if x == y => {},
                _ => {
                    d.push(i as u32);
                    d.push(match self.buffer.pixels[i] {
                        Some(x) => x as u32,
                        None => 0u32,
                    });
                    self.diff_count += 1;

                }
            }
        }
        self.frame.fill(if let Some(x) = self.background { x } else { 0 });
        std::mem::swap(&mut self.frame, &mut self.buffer);
        d.as_ptr()
    }

    pub fn diff_count(&self) -> u32 {
        self.diff_count
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.frame)?;
        Ok(())
    }
}