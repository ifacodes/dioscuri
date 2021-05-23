mod app;
#[allow(dead_code)]
mod validation;

#[macro_use]
extern crate lazy_static;

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
        ResizedView, ScrollView, TextArea, TextView,
    },
    Cursive, Rect,
};
use directories_next::ProjectDirs;
use pancurses::{initscr, resize_term};
use rustls::{ClientConfig, Session};
use std::{
    borrow::Cow,
    io::{self, Read, Write},
    sync::Arc,
};
use std::{net::TcpStream, path::PathBuf};
use validation::TOFUVerifier;
use webpki::DNSNameRef;

lazy_static! {
    static ref DIRECTORY: Option<ProjectDirs> = ProjectDirs::from("com", "ifa", "gem");
    static ref SAVED_CERTS: PathBuf = DIRECTORY.clone().map_or_else(
        || std::env::current_dir().unwrap(),
        |d| d.data_dir().to_owned()
    );
}

// FIXME: Move to seperate file!
fn build_config<'a>() -> Result<Arc<ClientConfig>> {
    let mut config = ClientConfig::new();
    config
        .dangerous()
        .set_certificate_verifier(Arc::new(TOFUVerifier::new(&SAVED_CERTS)));
    Ok(Arc::new(config))
}

// FIXME: Move to seperate file!
#[allow(dead_code)]
fn do_tls_stuff() -> String {
    let rc_config = build_config().unwrap();
    let gemini_test = DNSNameRef::try_from_ascii_str("gemini.circumlunar.space").unwrap();

    let gemini_request = b"gemini://gemini.circumlunar.space/servers/\r\n";

    let mut client = rustls::ClientSession::new(&rc_config, gemini_test);
    let mut socket = TcpStream::connect("gemini.circumlunar.space:1965").unwrap();
    let mut stream = rustls::Stream::new(&mut client, &mut socket);

    stream.write(gemini_request).unwrap();

    while client.wants_read() {
        client.read_tls(&mut socket).unwrap();
        client.process_new_packets().unwrap();
    }
    // FIXME: why does this not always return the page text?
    let mut data = Vec::new();
    let _ = client.read_to_end(&mut data);
    let _status = String::from_utf8_lossy(&data);

    client.read_tls(&mut socket).unwrap();
    client.process_new_packets().unwrap();
    let mut data = Vec::new();
    let _ = client.read_to_end(&mut data);
    let content = String::from_utf8_lossy(&data);
    content.to_string()
}

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
    let app = App::default();

    let top_bar = PaddedView::lrtb(
        2,
        3,
        1,
        1,
        LinearLayout::horizontal()
            .child(Panel::new(
                button_without_brackets("<--", |e| {}).with_name("back_button"),
            ))
            .child(Panel::new(
                button_without_brackets("-->", |e| {}).with_name("forward_button"),
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

    let page_view = PaddedView::lrtb(2, 2, 1, 3, TextView::new(do_tls_stuff()).scrollable());

    let mut siv = cursive::default();

    siv.set_user_data(app);

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
        c.find_name::<EditView>("urlbar")
            .unwrap()
            .set_content(&c.user_data::<App>().expect("FUCK").url);
    });

    siv.run();

    Ok(())
}
