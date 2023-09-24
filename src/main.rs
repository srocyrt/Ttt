mod screen;

use screen::{ BitMap, Screen };

pub fn main() -> () {
    let mut map = Screen::new(256, 64);
    let w1 = BitMap::white_noise(20, 12);
    let w2 = BitMap::white_noise(15, 8);
    let w3 = BitMap::white_noise(47, 30);
    let w4 = BitMap::white_noise(12, 20);
    let w5 = BitMap::white_noise(74, 10);
    map.paint(w1, 15, 47);
    map.paint(w2, 65, 21);
    map.paint(w3, 12, 5);
    map.paint(w4, 95, 9);
    map.paint(w5, 32, 12);
    // println!("{:?}", map.pixels);
    println!("{}", map);
}
