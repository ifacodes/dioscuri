mod validation;

#[macro_use]
extern crate lazy_static;

use anyhow::Result;
use directories_next::ProjectDirs;
use rustls::{ClientConfig, Session};
use std::{
    io::{Read, Write},
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

fn build_config<'a>() -> Result<Arc<ClientConfig>> {
    let mut config = ClientConfig::new();
    config
        .dangerous()
        .set_certificate_verifier(Arc::new(TOFUVerifier::new(&SAVED_CERTS)));
    Ok(Arc::new(config))
}

pub fn main() -> Result<()> {
    let rc_config = build_config().unwrap();
    let gemini_test = DNSNameRef::try_from_ascii_str("gemini.circumlunar.space").unwrap();

    let gemini_request = b"gemini://gemini.circumlunar.space/\r\n";

    let mut client = rustls::ClientSession::new(&rc_config, gemini_test);
    let mut socket = TcpStream::connect("gemini.circumlunar.space:1965").unwrap();
    let mut stream = rustls::Stream::new(&mut client, &mut socket);

    stream.write(gemini_request).unwrap();

    while client.wants_read() {
        client.read_tls(&mut socket).unwrap();
        client.process_new_packets().unwrap();
    }
    let mut data = Vec::new();
    let _ = client.read_to_end(&mut data);
    let status = String::from_utf8_lossy(&data);
    println!("{}", status);
    Ok(())
}
