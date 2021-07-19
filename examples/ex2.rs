use fltk::{prelude::*, *};
use fltk_flex::{Flex, FlexType};

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default().with_size(640, 480);
    let mut col = Flex::new(5, 5, 630, 470, None);
    col.set_type(FlexType::Column);
    {
        let mut row = Flex::default();
        row.set_type(FlexType::Row);
        {
            let _cancel = create_button("Cancel");
            let frame = frame::Frame::default().with_size(120, 10).with_label("Box");
            let _ok = create_button("OK");
            let _input = input::Input::default().with_size(120, 10);
            let mut col2 = Flex::default().with_size(130, 130);
            col2.set_type(FlexType::Column);
            {
                let top = create_button("Top");
                col2.resizable(&top);
                let _bottom = create_button("Bottom");
                col2.end();
            }
            row.resizable(&frame);
            row.end();
        }
        col.set_size(&mut *create_middle(), 30);
        let _ub1 = create_button("Something1");
        row = Flex::default();
        row.set_type(FlexType::Row);
        {
            let mut cancel = create_button("Cancel");
            let mut ok = create_button("OK");
            let _input = input::Input::default().with_size(120, 10);
            row.set_size(&mut cancel, 100);
            row.set_size(&mut ok, 100);
            row.end();
        }
        let _ub2 = create_button("Something2");
        col.set_size(&mut *row, 30);
        col.end();
    }
    win.resizable(&*col);
    win.end();
    win.show();
    a.run().unwrap();
}

fn create_middle() -> Flex {
    let mut row = Flex::default();
    row.set_type(FlexType::Row);
    {
        let _cancel = create_button("Cancel");
        let mut frame = frame::Frame::default().with_size(120, 10).with_label("Box");
        let _ok = create_button("OK");
        let _input = input::Input::default().with_size(120, 10);
        let mut col2 = Flex::default();
        col2.set_type(FlexType::Column);
        {
            create_button("Top");
            col2.end();
        }
        row.set_size(&mut frame, 30);
        row.set_size(&mut *col2, 100);
        row.end();
    }
    row
}

fn create_button(caption: &'static str) -> button::Button {
    let mut rtn = button::Button::default()
        .with_size(120, 30)
        .with_label(caption);
    rtn.set_color(enums::Color::from_rgb(225, 225, 225));
    rtn
}
