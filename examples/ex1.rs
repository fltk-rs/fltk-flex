use fltk::{prelude::*, *};
use fltk_flex::Flex;

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default().with_size(400, 300);
    let mut flex = Flex::default().size_of_parent().column();
    let _expanding = button::Button::default().with_label("Expanding");
    let mut normal = button::Button::default().with_label("Normal");
    flex.set_size(&mut normal, 30);
    flex.end();
    win.end();
    win.make_resizable(true);
    win.show();
    a.run().unwrap();
}