use std::fmt;
use wasm_bindgen::prelude::*;

// pub const WIDTH: usize = 256;
// pub const HEIGHT: usize = 64;

type Color = u32;

#[wasm_bindgen]
pub struct BitMap {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl std::ops::Index<usize> for BitMap {
    type Output = Color;
    fn index(&self, index: usize) -> &Self::Output {
        &self.pixels[index]
    }
}

impl std::ops::IndexMut<usize> for BitMap {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pixels[index]
    }
}

#[wasm_bindgen]
impl BitMap {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![0; width * height];
        assert_eq!(width * height, pixels.capacity());
        BitMap {
            width,
            height,
            pixels,
        }
    }

    pub fn fill(&mut self, color: Color) {
        self.pixels.fill(color);
    }

    pub fn white_noise(width: usize, height: usize) -> Self {
        let pixels = std::iter::repeat_with(|| rand::random())
            .take(width * height)
            .collect();
        BitMap {
            width,
            height,
            pixels,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, c: Color) {
        if x > self.width {
            unreachable!()
        };
        if y > self.height {
            unreachable!()
        };
        self.pixels[self.width * y + x] = c;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        if x > self.width {
            unreachable!()
        };
        if y > self.height {
            unreachable!()
        };
        self.pixels[self.width * y + x]
    }
}

// ▄█▀
impl fmt::Display for BitMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.height % 2 != 0 {
            todo!()
        }
        for row in (0..self.height).step_by(2) {
            for col in 0..self.width {
                let fore = self.get_pixel(col, row).to_le_bytes();
                let back = self.get_pixel(col, row + 1).to_le_bytes();
                write!(
                    f,
                    "\u{001b}[38;2;{:03};{:03};{:03};48;2;{:03};{:03};{:03}m▀\u{001b}[0m",
                    fore[3], fore[2], fore[1], back[3], back[2], back[1]
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
    background: Option<Color>,
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

    pub fn with_bg(width: usize, height: usize, bg: Color) -> Self {
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

    pub fn paint(&mut self, sprite: &BitMap, x: usize, y: usize) {
        let height_bound: usize = if y + sprite.height > self.height {
            self.height - y
        } else {
            sprite.height
        };
        let width_bound: usize = if x + sprite.width > self.width {
            self.width - x
        } else {
            sprite.width
        };
        for offset_y in 0..height_bound {
            for offset_x in 0..width_bound {
                // ToFix mask
                self.buffer.set_pixel(
                    x + offset_x,
                    y + offset_y,
                    sprite.get_pixel(offset_x, offset_y),
                )
            }
        }
    }

    pub fn next(&mut self) -> String {
        self.frame
            .fill(if let Some(x) = self.background { x } else { 0 });
        std::mem::swap(&mut self.frame, &mut self.buffer);
        format!("{}", self.frame)
    }

    pub fn diff(&mut self) -> *const u8 {
        let mut d: Vec<u8> = vec![];
        self.diff_count = 0;

        for row in (0..self.height).step_by(2) {
            for col in 0..self.width {
                match (
                    self.buffer.get_pixel(col, row),
                    self.frame.get_pixel(col, row),
                    self.buffer.get_pixel(col, row + 1),
                    self.frame.get_pixel(col, row + 1),
                ) {
                    (b1, f1, b2, f2) if b1 == f1 && b2 == f2 => {
                        continue;
                    }
                    (b1, _, b2, _) => {
                        d.push(row as u8);
                        d.push(col as u8);
                        let b = b1.to_le_bytes();
                        d.push(b[3]);
                        d.push(b[2]);
                        d.push(b[1]);
                        let b = b2.to_le_bytes();
                        d.push(b[3]);
                        d.push(b[2]);
                        d.push(b[1]);
                        self.diff_count += 1;
                    }
                }
            }
        }

        self.frame
            .fill(if let Some(x) = self.background { x } else { 0 });
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
