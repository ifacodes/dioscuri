use super::validation::TOFUVerifier;
use cursive::views::TextContent;
use rustls::{ClientConfig, Session};
use std::{
    borrow::Cow,
    io::{self, Read, Write},
    sync::Arc,
};
use std::{net::TcpStream, path::PathBuf};
use webpki::DNSNameRef;
pub enum InputMode {
    UrlInput,
    NoInput,
}
use anyhow::*;

use directories_next::ProjectDirs;

lazy_static! {
    static ref DIRECTORY: Option<ProjectDirs> = ProjectDirs::from("com", "ifa", "gem");
    static ref SAVED_CERTS: PathBuf = DIRECTORY.clone().map_or_else(
        || std::env::current_dir().unwrap(),
        |d| d.data_dir().to_owned()
    );
}

pub struct App {
    pub url: String,
    pub previous_urls: Vec<String>,
    pub page: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            url: "gemini.circumlunar.space".to_owned(),
            previous_urls: Vec::new(),
            page: String::new(),
        }
    }
}

impl App {
    pub fn update_url(&mut self, new_url: String) {
        self.url = new_url;
    }

    pub fn fetch_page(&mut self) {
        let rc_config = App::build_config().unwrap();
        let dns_name = DNSNameRef::try_from_ascii_str(self.url.as_str()).unwrap();

        let gemini_request = format!("gemini://{}/\r\n", self.url);

        let mut client = rustls::ClientSession::new(&rc_config, dns_name);
        let mut socket = TcpStream::connect(&format!("{}:1965", self.url)).unwrap();
        let mut stream = rustls::Stream::new(&mut client, &mut socket);

        stream.write(gemini_request.as_bytes()).unwrap();

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
        self.page = content.to_string();
    }

    pub fn fetch_page2(&mut self) {
        let rc_config = App::build_config().unwrap();
        let dns_name = DNSNameRef::try_from_ascii_str("gemini.circumlunar.space").unwrap();

        let gemini_request = format!("gemini://gemini.circumlunar.space/servers/\r\n");

        let mut client = rustls::ClientSession::new(&rc_config, dns_name);
        let mut socket = TcpStream::connect(&format!("gemini.circumlunar.space:1965")).unwrap();
        let mut stream = rustls::Stream::new(&mut client, &mut socket);

        stream.write(gemini_request.as_bytes()).unwrap();

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
        self.page = content.to_string();
    }

    fn build_config<'a>() -> Result<Arc<ClientConfig>> {
        let mut config = ClientConfig::new();
        config
            .dangerous()
            .set_certificate_verifier(Arc::new(TOFUVerifier::new(&SAVED_CERTS)));
        Ok(Arc::new(config))
    }
}

struct Url {
    scheme: String,
    address: String,
    port: String,
    path: String,
}

impl Url {
    /*fn new(url: &str) -> Self {
        let mut url_string = url.to_string();
        // Example gemini.circumlunar.space/servers/

        //if let Some(index) = url_string.find("://")
    }*/
}
