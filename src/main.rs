mod screen;

// use std::fs::File;

// use screen::{BitMap, Screen};

pub fn main() -> () {
    let file = std::fs::File::open("./node/rickroll-roll.gif").expect("Unable to open the file.");
    let mut screen = ascii_displayer::GiffDisplayer::new(file);
    let mut i = 0;
    while let Ok(()) = screen.next() {
        i += 1;
        println!("{}", i);
    }
}
