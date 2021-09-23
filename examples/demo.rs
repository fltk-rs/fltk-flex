use fltk::{prelude::*, *};
use fltk_flex::Flex;

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default().with_size(640, 480);
    let mut col = Flex::new(5, 5, 630, 470, None).column();
    {
        let mut row = Flex::default().row();
        {
            let _cancel = create_button("Cancel");
            let frame = frame::Frame::default().with_label("Box");
            let _ok = create_button("OK");
            let _input = input::Input::default();
            let col2 = Flex::default().column();
            {
                let top = create_button("Top");
                col2.resizable(&top);
                let _bottom = create_button("Bottom");
                col2.end();
            }
            row.resizable(&frame);
            row.end();
        }
        col.set_size(&mut create_middle(), 30);
        let _ub1 = create_button("Something1");
        row = Flex::default().row();
        {
            let mut cancel = create_button("Cancel");
            let mut ok = create_button("OK");
            let _input = input::Input::default();
            row.set_size(&mut cancel, 100);
            row.set_size(&mut ok, 100);
            row.end();
        }
        let _ub2 = create_button("Something2");
        col.set_size(&mut row, 30);
        col.end();
    }
    win.resizable(&col);
    win.end();
    win.show();
    a.run().unwrap();
}

fn create_middle() -> Flex {
    let mut row = Flex::default().row();
    {
        let _cancel = create_button("Cancel");
        let mut frame = frame::Frame::default().with_label("Box");
        let _ok = create_button("OK");
        let _input = input::Input::default();
        let mut col2 = Flex::default().column();
        {
            create_button("Top");
            col2.end();
        }
        row.set_size(&mut frame, 30);
        row.set_size(&mut col2, 100);
        row.end();
    }
    row
}

fn create_button(caption: &str) -> button::Button {
    let mut btn = button::Button::default()
        .with_label(caption);
    btn.set_color(enums::Color::from_rgb(225, 225, 225));
    btn
}
