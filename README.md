# fltk-flex

A Rust port of [FL_Flex](https://github.com/osen/FL_Flex), which provides a flexbox widget for FLTK.

## Usage
```toml
[dependencies]
fltk = "1"
fltk-flex = "0.1"
```

## Example
```rust
use fltk::{prelude::*, *};
use fltk_flex::{Flex, FlexType};

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default().with_size(400, 300);
    let mut flex = Flex::new(0, 0, 400, 300, None);
    flex.set_type(FlexType::Column);
    let expanding = button::Button::default().with_label("Expanding");
    let mut normal = button::Button::default().with_label("Normal");
    flex.set_size(&mut normal, 30);
    flex.end();
    win.end();
    win.show();
    a.run().unwrap();
}
```