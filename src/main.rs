mod app;
mod ui;
#[allow(dead_code)]
mod validation;

#[macro_use]
extern crate lazy_static;

use std::{
    env::current_dir,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use app::App;
use cursive::{
    align::HAlign,
    event::Key,
    theme::{ColorStyle, PaletteColor},
    traits::Scrollable,
    view::{Boxable, Identifiable, SizeConstraint},
    views::{
        BoxView, Button, Dialog, DummyView, EditView, FixedLayout, LinearLayout, PaddedView, Panel,
        ResizedView, ScrollView, TextArea, TextContent, TextView, ViewRef,
    },
    Cursive, CursiveRunnable, Rect, With,
};

pub fn main() -> Result<()> {
    env_logger::init();
    let mut cursive = cursive::default();
    let mut app = App::new(&mut cursive);
    app.run();

    Ok(())
}
