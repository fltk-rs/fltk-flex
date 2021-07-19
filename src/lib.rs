/*!
# fltk-flex

A Rust port of [FL_Flex](https://github.com/osen/FL_Flex), which provides a flexbox widget for fltk-rs.

## Usage
```toml,no_run
[dependencies]
fltk = "1"
fltk-flex = "0.1"
```

## Example
```rust,no_run
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
*/

use fltk::{
    group::Group,
    prelude::{GroupExt, WidgetBase, WidgetExt, WidgetType},
    widget::Widget,
};
use std::{
    mem,
    ops::{Deref, DerefMut},
};

/// Defines Flex types
#[repr(i32)]
#[derive(fltk_derive::WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum FlexType {
    /// row direction
    Row = 0,
    /// column direction
    Column,
}

/**
    a Flexbox widget
    # Example
    ```rust,no_run
    use fltk::{prelude::*, *};
    use fltk_flex::{Flex, FlexType};
    let mut col = Flex::new(0, 0, 400, 300, None);
    col.set_type(FlexType::Column);
    let expanding = button::Button::default().with_label("Expanding");
    let mut normal = button::Button::default().with_label("Normal");
    col.set_size(&mut normal, 30);
    col.end();
    ```
*/
#[derive(Debug, Clone)]
pub struct Flex {
    grp: Group,
    dir: FlexType,
    margin: i32,
    pad: i32,
    setsized: Vec<Widget>,
}

// Code translated from https://github.com/osen/FL_Flex
impl Flex {
    /// Create a new Flex widget
    pub fn new<T: Into<Option<&'static str>>>(x: i32, y: i32, w: i32, h: i32, label: T) -> Flex {
        let dir = FlexType::Row;
        let margin = 0;
        let pad = 5;
        let grp = Group::new(x, y, w, h, label);
        Self {
            grp,
            dir,
            margin,
            pad,
            setsized: Vec::new(),
        }
    }

    /// Create a default initialized Flex widget
    pub fn default() -> Self {
        let dir = FlexType::Row;
        let margin = 0;
        let pad = 5;
        let grp = Group::default().size_of_parent();
        Self {
            grp,
            dir,
            margin,
            pad,
            setsized: Vec::new(),
        }
    }

    /// Create a flex with size
    pub fn with_size(mut self, w: i32, h: i32) -> Self {
        self.grp.set_size(w, h);
        self
    }

    /// Set the direction
    pub fn set_type<T: WidgetType>(&mut self, typ: T) {
        self.dir = FlexType::from_i32(typ.to_i32());
    }

    /// Get the direction
    pub fn get_type<T: WidgetType>(&self) -> T {
        T::from_i32(self.dir.to_i32())
    }

    /// End the Flex widget
    pub fn end(&mut self) {
        self.grp.end();
        self.resize(self.grp.x(), self.grp.y(), self.grp.w(), self.grp.h());
    }

    /// Set the size of the widget
    pub fn set_size<W: WidgetExt>(&mut self, wid: &mut W, size: i32) {
        wid.resize(0, 0, size, size);
        for i in 0..self.setsized.len() {
            if unsafe { self.setsized[i].as_widget_ptr() == wid.as_widget_ptr() } {
                return;
            }
        }
        self.setsized
            .push(unsafe { Widget::from_widget_ptr(wid.as_widget_ptr()) });
    }

    /// Resize the Flex widget
    pub fn resize(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.widget_resize(x, y, w, h);
        if self.dir == FlexType::Column {
            self.resize_col(x, y, w, h);
        } else {
            self.resize_row(x, y, w, h);
        }
    }

    fn is_set_size<W: WidgetExt>(&self, wid: &W) -> bool {
        for i in 0..self.setsized.len() {
            if unsafe { wid.as_widget_ptr() == self.setsized[i].as_widget_ptr() } {
                return true;
            }
        }
        return false;
    }

    fn resize_row(&mut self, x: i32, y: i32, w: i32, h: i32) {
        let cc = self.grp.children();
        let mut pad_w = w - self.margin * 2;
        for _i in 0..cc {
            pad_w -= 5;
        }
        let mut cx = x + self.margin;
        let mut nrs = 0;
        for i in 0..cc {
            let c = self.grp.child(i).unwrap();

            if self.is_set_size(&c) {
                nrs += c.w();
            }
        }
        for i in 0..cc {
            let mut c = self.grp.child(i).unwrap();

            if self.is_set_size(&c) {
                c.resize(cx, y + self.margin, c.w(), h - self.margin * 2);
            } else {
                c.resize(
                    cx,
                    y + self.margin,
                    (pad_w - nrs) / (cc - self.setsized.len() as i32),
                    h - self.margin * 2,
                );
            }

            cx += c.w() + self.pad;
        }
    }

    fn resize_col(&mut self, x: i32, y: i32, w: i32, h: i32) {
        let cc = self.grp.children();
        if cc - self.setsized.len() as i32 == 0 {
            return;
        }
        let mut pad_h = h - self.margin * 2;
        for _i in 0..cc {
            pad_h -= self.pad;
        }
        let mut cy = y + self.margin;
        let mut nrs = 0;
        for i in 0..cc {
            let c = self.grp.child(i).unwrap();

            if self.is_set_size(&c) {
                nrs += c.h();
            }
        }
        for i in 0..cc {
            let mut c = self.grp.child(i).unwrap();
            if self.is_set_size(&c) {
                c.resize(x + self.margin, cy, w - self.margin * 2, c.h());
            } else {
                c.resize(
                    x + self.margin,
                    cy,
                    w - self.margin * 2,
                    (pad_h - nrs) / (cc - self.setsized.len() as i32),
                );
            }

            cy += c.h() + self.pad;
        }
    }
}

impl Deref for Flex {
    type Target = Group;

    fn deref(&self) -> &Self::Target {
        &self.grp
    }
}

impl DerefMut for Flex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grp
    }
}
