use fltk::{prelude::*, *};
use fltk_flex::Flex;

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default().with_size(400, 300);
    {
        let mut flex = Flex::default().column();
        {
            frame::Frame::default(); // filler
            let mut normal1 = button::Button::default().with_label("Normal1");
            let mut normal2 = button::Button::default().with_label("Normal2");
            frame::Frame::default(); // filler
            flex.set_size(&mut normal1, 30);
            flex.set_size(&mut normal2, 30);
            flex.end();
            normal1.set_callback(|b| println!("{}", b.label()));
            normal2.set_callback(|b| println!("{}", b.label()));
        }
        win.end();
        win.make_resizable(true);
        win.show();
    }
    a.run().unwrap();
}