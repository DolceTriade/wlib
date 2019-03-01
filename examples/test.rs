extern crate wlib;
extern crate x11;

use x11::xlib;

#[derive(Copy, Clone)]
enum Attr {
    X,
    Y,
}

fn main() {
    let disp = wlib::Display::open().unwrap();
    let pos = disp.pointer().unwrap();
    let root = disp.screen().unwrap().root().unwrap();
    for child in root.children().unwrap() {
        println!("class = {:?}", child.class_name());
    }
}
