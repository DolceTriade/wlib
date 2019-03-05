extern crate wlib;
extern crate x11;

fn main() {
    let disp = wlib::Display::open().unwrap();
    let mut root = disp.screen().unwrap().root().unwrap();
    print_children(&root);
}

fn print_children(window: &wlib::Window) {
    match window.children() {
        Ok(children) => {
            for child in children {
                print_children(&child);
            }
        }
        Err(_) => {}
    }
    print_window(window);
}

fn print_window(window: &wlib::Window) {
    let f = window.content();
    println!(
        "class = {:?} {} {} {} {} {:?} {:?}",
        window.class_name(),
        &f.p.x,
        &f.p.y,
        &f.r.w,
        &f.r.h,
        window.ignored(),
        window.visible(),
    );
}
