use std::{convert::TryInto, sync::mpsc};

use crossbeam_channel::{unbounded, Receiver, Sender};
use cursive::{
    align::HAlign,
    traits::{Boxable, Nameable, Scrollable},
    view::SizeConstraint,
    views::{Button, EditView, LinearLayout, PaddedView, Panel, ResizedView, TextView},
    Cursive, CursiveRunnable, CursiveRunner,
};

use crate::app::{App, AppMessage};

fn button_without_brackets<S, F>(label: S, cb: F) -> Button
where
    S: Into<String>,
    F: 'static + Fn(&mut Cursive),
{
    let mut button = Button::new("", cb);
    button.set_label_raw(label);
    button
}
pub struct UISystem<'a> {
    siv: CursiveRunner<&'a mut Cursive>,
    rx: Receiver<UIMessage>,
    pub tx: Sender<UIMessage>,
    app_send: Sender<AppMessage>,
}

pub enum UIMessage {
    UpdateText(String),
}

impl<'a> UISystem<'a> {
    pub fn new(siv: CursiveRunner<&'a mut Cursive>, app_send: Sender<AppMessage>) -> Self {
        let (tx, rx) = unbounded::<UIMessage>();
        let mut ui = Self {
            siv,
            rx,
            tx,
            app_send,
        };

        let tx_clone = ui.app_send.clone();
        let top_bar = PaddedView::lrtb(
            2,
            3,
            1,
            1,
            LinearLayout::horizontal()
                .child(Panel::new(
                    button_without_brackets("<--", |c| todo!()).with_name("back_button"),
                ))
                .child(Panel::new(
                    button_without_brackets("-->", |c| todo!()).with_name("forward_button"),
                ))
                .child(PaddedView::lrtb(
                    2,
                    0,
                    0,
                    0,
                    Panel::new(
                        EditView::new()
                            .on_submit_mut(move |c, str| {
                                // TODO: tell the app to add to current url
                                tx_clone
                                    .clone()
                                    .send(AppMessage::UpdateURL(str.to_string()))
                                    .unwrap();
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
            TextView::new("placeholder")
                .with_name("pageview")
                .scrollable(),
        );

        ui.siv.add_fullscreen_layer(
            LinearLayout::vertical()
                .child(ResizedView::new(
                    SizeConstraint::Full,
                    SizeConstraint::Free,
                    top_bar,
                ))
                .child(page_view),
        );

        let tx_clone = ui.app_send.clone();
        ui.siv.add_global_callback('q', move |c| {
            let input = c.find_name::<EditView>("urlbar").unwrap();
            tx_clone
                .send(AppMessage::UpdateURL(input.get_content().to_string()))
                .unwrap();
        });

        ui
    }
    pub fn step(&mut self) -> bool {
        if !self.siv.is_running() {
            return false;
        }
        while let Some(message) = self.rx.try_iter().next() {
            match message {
                UIMessage::UpdateText(text) => {
                    let mut display = self.siv.find_name::<TextView>("pageview").unwrap();
                    display.set_content(text);
                }
            }
        }

        self.siv.step();
        true
    }
}
