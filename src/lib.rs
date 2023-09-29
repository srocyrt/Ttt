mod screen;

use std::io::Cursor;

// use screen::BitMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GifDisplayer {
    decoder: gif::Decoder<Cursor<Vec<u8>>>,
    global_palette: Vec<u8>,
    scale: usize,
    width: usize,
    height: usize,
    screen: screen::Screen,
    bms: Vec<screen::BitMap>,
}

pub struct GiffDisplayer {
    decoder: gif::Decoder<std::fs::File>,
    global_palette: Vec<u8>,
    scale: usize,
    width: usize,
    height: usize,
    screen: screen::Screen,
}

impl GiffDisplayer {
    pub fn new(file: std::fs::File) -> Self {
        let options = gif::DecodeOptions::new();
        let decoder = options.read_info(file).unwrap();
        let global_palette: Vec<u8> = decoder
            .global_palette()
            .unwrap()
            .into_iter()
            .cloned()
            .collect();
        let width = decoder.width() as usize;
        let height = decoder.height() as usize;
        let scale: usize = std::cmp::max(width / 64 + 1, height / 64 + 1);
        let width = (width - width % scale) as usize;
        let height = (height - height % scale) as usize;
        // let screen = screen::Screen::with_bg(width / scale, height / scale, 0x2C2C2CFF);
        let screen = screen::Screen::with_bg(width / scale, height / scale, 0xFFFFFFFF);
        GiffDisplayer {
            decoder,
            global_palette,
            scale,
            width,
            height,
            screen,
        }
    }

    pub fn to_string(&mut self) -> String {
        format!("{}", self.screen)
    }

    pub fn diff(&mut self) -> *const u8 {
        self.screen.diff()
    }

    pub fn diff_count(&self) -> u32 {
        self.screen.diff_count()
    }

    pub fn next(&mut self) -> Result<(), ()>{
        // let mut bitmap = screen::BitMap::new(self.width, self.height);
        if let Some(frame) = self.decoder.read_next_frame().unwrap() {
            print!("[{:06}, {:06}]", frame.left, frame.top);
            print!("[{:06}, {:06}]", frame.width, frame.height);
            print!("[{:06}, {:06}]\n", frame.buffer.len(), frame.delay);
            // for x in (0..self.width).step_by(self.scale) {
            //     for y in (0..self.height).step_by(self.scale) {
            //         let mut color: (u32, u32, u32, u32) = (0, 0, 0, 0);
            //         for dx in 0..self.scale {
            //             for dy in 0..self.scale {
            //                 if let Some(offset) =
            //                     frame.buffer.get((y + dy) * frame.width as usize + (x + dx))
            //                 {
            //                     let offset = *offset as usize;
            //                     if let Some(ref p) = frame.palette {
            //                         color.0 += p[offset * 3 + 0] as u32;
            //                         color.1 += p[offset * 3 + 1] as u32;
            //                         color.2 += p[offset * 3 + 2] as u32;
            //                     } else {
            //                         color.0 += self.global_palette[offset * 3 + 0] as u32;
            //                         color.1 += self.global_palette[offset * 3 + 1] as u32;
            //                         color.2 += self.global_palette[offset * 3 + 2] as u32;
            //                     }
            //                 } else {
            //                     println!("width: {}, height: {}, size: {}", frame.width, frame.height, frame.buffer.len());
            //                     println!("left: {}, top: {}", frame.left, frame.top);
            //                     println!("x: {} dx: {} y: {} dy: {}", x, dx, y, dy);
            //                     println!("index: {}\n", (y + dy) * frame.width as usize + (x + dx));
            //                 }
            //             }
            //         }
            //         let color = match color {
            //             // Tofix
            //             (r, g, b, _) => [0, (b >> 2) as u8, (g >> 2) as u8, (r >> 2) as u8],
            //         };
            //         bitmap.set_pixel(x / self.scale, y / self.scale, u32::from_le_bytes(color));
            //     }
            // }
            Ok(())
        } else {
            Err(())
        }
        // self.screen.paint(bitmap, 0, 0);
        // self.screen.next();
    }
}

#[wasm_bindgen]
impl GifDisplayer {
    pub fn new(raw_data: &[u8]) -> Self {
        let raw_data: Vec<u8> = raw_data.to_owned();
        let options = gif::DecodeOptions::new();
        let decoder = options.read_info(Cursor::new(raw_data.to_owned())).unwrap();
        let global_palette: Vec<u8> = decoder
            .global_palette()
            .unwrap()
            .into_iter()
            .cloned()
            .collect();
        let width = decoder.width() as usize;
        let height = decoder.height() as usize;
        let scale: usize = std::cmp::max(width / 64 + 1, height / 64 + 1);
        let width = (width - width % scale) as usize;
        let height = (height - height % scale) as usize;
        // let screen = screen::Screen::with_bg(width / scale, height / scale, 0x2C2C2CFF);
        let screen = screen::Screen::with_bg(width / scale, height / scale, 0xFFFFFFFF);
        let bms = vec![];
        GifDisplayer {
            decoder,
            global_palette,
            scale,
            width,
            height,
            screen,
            bms,
        }
    }

    pub fn to_string(&mut self) -> String {
        format!("{}", self.screen)
    }

    pub fn diff(&mut self) -> *const u8 {
        self.screen.diff()
    }

    pub fn diff_count(&self) -> u32 {
        self.screen.diff_count()
    }

    pub fn next(&mut self) {
        if let Some(frame) = self.decoder.read_next_frame().unwrap() {
            let mut bitmap = screen::BitMap::new(self. width / self.scale, self.height / self.scale);
            // let gif::Frame { left, top, width, height, ..} = frame;

            for x in (0..self.width).step_by(self.scale) {
                for y in (0..self.height).step_by(self.scale) {
                    let mut color: (u32, u32, u32, u32) = (0, 0, 0, 0);
                    for dx in 0..self.scale {
                        for dy in 0..self.scale {
                            if let Some(offset) =
                                frame.buffer.get((y + dy) * frame.width as usize + (x + dx))
                            {
                                let offset = *offset as usize;
                                if let Some(ref p) = frame.palette {
                                    color.0 += p[offset * 3 + 0] as u32;
                                    color.1 += p[offset * 3 + 1] as u32;
                                    color.2 += p[offset * 3 + 2] as u32;
                                } else {
                                    color.0 += self.global_palette[offset * 3 + 0] as u32;
                                    color.1 += self.global_palette[offset * 3 + 1] as u32;
                                    color.2 += self.global_palette[offset * 3 + 2] as u32;
                                }
                            } else {
                                panic!("no");
                            }
                        }
                    }
                    let color = match color {
                        // Tofix
                        (r, g, b, _) => [0, (b >> 6) as u8, (g >> 6) as u8, (r >> 6) as u8],
                    };
                    bitmap.set_pixel(x / self.scale, y / self.scale, u32::from_le_bytes(color));
                }
            }
            self.bms.push(bitmap);
        } else {
            self.bms.rotate_left(1);
        }
        self.screen.paint(&self.bms.last().unwrap(), 0, 0);
        // self.screen.next();
    }
}

// #[wasm_bindgen]
// pub fn gif(ptr: &[u8], id: usize) -> BitMap {
//     let options = gif::DecodeOptions::new();
//     // options.set_color_output(gif::ColorOutput::RGBA);
//     let mut decoder = options.read_info::<&[u8]>(ptr).unwrap();
//     let global_palette: Vec<u8> = decoder
//         .global_palette()
//         .unwrap()
//         .into_iter()
//         .cloned()
//         .collect();
//     let mut bm = screen::BitMap::new(128, 128);
//     for _ in 0..id {
//         decoder.read_next_frame();
//     }
//     // decoder.read_next_frame();
//     if let Some(frame) = decoder.read_next_frame().unwrap() {
//         let w: usize = (frame.width - (frame.width % 4)) as usize;
//         let h: usize = (frame.height - (frame.height % 4)) as usize;
//         for x in (0..w).step_by(4) {
//             for y in (0..h).step_by(4) {
//                 let mut color: (u32, u32, u32, u32) = (0, 0, 0, 0);
//                 for dx in 0..4 {
//                     for dy in 0..4 {
//                         if let Some(offset) =
//                             frame.buffer.get((y + dy) * frame.width as usize + (x + dx))
//                         {
//                             let offset = *offset as usize;
//                             if let Some(ref p) = frame.palette {
//                                 // return (p.len()).to_string();
//                                 color.0 += p[offset * 3 + 0] as u32;
//                                 color.1 += p[offset * 3 + 1] as u32;
//                                 color.2 += p[offset * 3 + 2] as u32;
//                             } else {
//                                 color.0 += global_palette[offset * 3 + 0] as u32;
//                                 color.1 += global_palette[offset * 3 + 1] as u32;
//                                 color.2 += global_palette[offset * 3 + 2] as u32;
//                                 // return format!("{:?}", color);
//                             }
//                         } else {
//                             panic!("no");
//                         }
//                     }
//                 }
//                 let color = match color {
//                     (r, g, b, a) => [0, (b >> 4) as u8, (g >> 4) as u8, (r >> 4) as u8],
//                 };
//                 bm.set_pixel(x / 4, y / 4, u32::from_le_bytes(color));
//             }
//         }
//     }
//     bm
// }

// #[wasm_bindgen]
// pub fn gif(ptr: &[u8], id: usize) ->  {

// }
