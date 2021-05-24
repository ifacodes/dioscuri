mod app;
#[allow(dead_code)]
mod validation;

#[macro_use]
extern crate lazy_static;

use std::sync::{Arc, Mutex};

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
        ResizedView, ScrollView, TextArea, TextContent, TextView,
    },
    Cursive, Rect, With,
};

// FIXME: Move to seperate file!

fn button_without_brackets<S, F>(label: S, cb: F) -> Button
where
    S: Into<String>,
    F: 'static + Fn(&mut Cursive),
{
    let mut button = Button::new("", cb);
    button.set_label_raw(label);
    button
}

fn url_submit(app: &mut App, input: &str) {
    app.update_url(input.to_string());
}

pub fn main() -> Result<()> {
    env_logger::init();
    let app = Arc::new(Mutex::new(App::default()));
    {
        app.lock().unwrap().fetch_page();
    }
    let top_bar = PaddedView::lrtb(
        2,
        3,
        1,
        1,
        LinearLayout::horizontal()
            .child(Panel::new(
                button_without_brackets("<--", |c| {
                    c.with_user_data(|_data: &mut App| todo!());
                })
                .with_name("back_button"),
            ))
            .child(Panel::new(
                button_without_brackets("-->", |c| {
                    c.with_user_data(|_data: &mut App| todo!());
                })
                .with_name("forward_button"),
            ))
            .child(PaddedView::lrtb(
                2,
                0,
                0,
                0,
                Panel::new(
                    EditView::new()
                        .on_submit(|c, str| {
                            c.with_user_data(|data: &mut App| data.update_url(str.to_string()));
                        })
                        .with_name("urlbar"),
                )
                .title_position(HAlign::Left)
                .title("URL")
                .resized(SizeConstraint::Full, SizeConstraint::Fixed(3)),
            )),
    );

    let page_view = PaddedView::lrtb(
        2,
        2,
        1,
        3,
        TextView::new(&app.lock().unwrap().page).scrollable(),
    );

    let mut siv = cursive::default();

    siv.set_user_data(app.clone());

    siv.add_fullscreen_layer(
        LinearLayout::vertical()
            .child(ResizedView::new(
                SizeConstraint::Full,
                SizeConstraint::Free,
                top_bar,
            ))
            .child(page_view),
    );
    siv.add_global_callback(Key::Esc, |c| {
        // When the user presses Escape, update the output view
        // with the contents of the input view.
        c.with_user_data(|data: &mut Arc<Mutex<App>>| {
            let mut locked = data.lock().unwrap();
            locked.update_url(String::from("gemini.circumlunar.space/servers"));
            locked.fetch_page2();
        });
    });

    siv.run();

    Ok(())
}
