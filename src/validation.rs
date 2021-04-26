use rustls::{Certificate, RootCertStore, ServerCertVerified, ServerCertVerifier, TLSError};
use std::sync::Arc;
use webpki::DNSNameRef;
use x509_parser::parse_x509_certificate;

pub struct TOFUVerifier {}

impl TOFUVerifier {
    pub fn new() -> Arc<Self> {
        Arc::new(TOFUVerifier {})
    }
}

impl ServerCertVerifier for TOFUVerifier {
    fn verify_server_cert(
        &self,
        _: &RootCertStore,
        presented_certs: &[Certificate],
        _hostname: DNSNameRef,
        _: &[u8],
    ) -> Result<ServerCertVerified, TLSError> {
        let cert = &presented_certs[0];
        let result = parse_x509_certificate(cert.as_ref());
        match result {
            Ok((rem, cert)) => {
                assert!(rem.is_empty());
                println! {"Certificate {:#?}", cert}
            }
            _ => panic!("parsing failed! {:?}", result),
        }
        Ok(ServerCertVerified::assertion())
    }
}
