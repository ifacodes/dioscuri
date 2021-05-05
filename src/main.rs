mod app;
#[allow(dead_code)]
mod gemini;
mod ui;
mod validation;

#[macro_use]
extern crate lazy_static;

use anyhow::Result;
use app::App;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use directories_next::ProjectDirs;
use rustls::{ClientConfig, Session};
use std::{
    io::{self, Read, Write},
    sync::Arc,
};
use std::{net::TcpStream, path::PathBuf};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
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
fn do_tls_stuff() {
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
    let status = String::from_utf8_lossy(&data);
    println!("{}", status);
}

fn draw_ui(app: &App) -> Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = ui::Events::new(250);

    loop {
        terminal.show_cursor()?;
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([Constraint::Length(3), Constraint::Length(1)].as_ref())
                .split(f.size());
            let input = Paragraph::new(app.url.as_ref())
                .style(match app.input_mode {
                    app::InputMode::NoInput => Style::default(),
                    app::InputMode::UrlInput => Style::default().fg(Color::Yellow),
                })
                .block(Block::default().borders(Borders::ALL).title("URL"));
            let page = Block::default().borders(Borders::ALL);
            f.render_widget(input, chunks[0]);

            f.render_widget(page, chunks[1]);
        })?;

        match events.next()? {
            ui::Event::Input(input) => {
                if input == ui::Input::Ctrl('c') {
                    break;
                }
                if input == ui::Input::Char('/') {
                    // TODO: Go to the search thingy here.
                }
            }
            ui::Event::Tick => {}
        }
    }

    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen)?;

    Ok(())
}
pub fn main() -> Result<()> {
    let app = App::default();
    draw_ui(&app)?;
    Ok(())
}
