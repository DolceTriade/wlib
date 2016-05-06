extern crate wtools;

fn run() -> wtools::OrErrorStr<()> {
    let disp = try!(wtools::Display::open());
    let scrn = try!(disp.screen());
    let root = try!(scrn.root());
    println!("Got {}x{} Screen with root = 0x{:x}", scrn.width(), scrn.height(), root.id());
    let win = try!(disp.window(0x00e00009));
    try!(win.position(10, 10));
    try!(win.resize(500, 400));
    Ok(())
}

fn main() {
   wtools::handle_error(run());
}
