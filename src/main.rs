use rustls::{Certificate, RootCertStore, ServerCertVerifier, Session};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpSocket, TcpStream};
mod validation;

fn main() {
    let mut config = rustls::ClientConfig::new();
    let mut danger_config = rustls::DangerousClientConfig { cfg: &mut config };
    let verifier = validation::TOFUVerifier::new();

    let rc_config = Arc::new(config);
    let gemini_test = webpki::DNSNameRef::try_from_ascii_str("gemini.circumlunar.space").unwrap();
    let mut client = rustls::ClientSession::new(&rc_config, test);

    let gemini_request = b"gemini:://gemini.circumlunar.space/servers/\r\n";

    // Step 1. connect
    let mut socket = TcpStream::connect("gemini://gemini.circumlunar.space:1965").await?;

    // Step 2. TLS Handshake
    client.write_tls(&mut socket).unwrap();

    println!("Hello, world!");
}
