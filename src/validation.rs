use anyhow::{anyhow, Result};
use ring::{digest, hmac::verify};
use rustls::{Certificate, RootCertStore, ServerCertVerified, ServerCertVerifier, TLSError};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use webpki::DNSNameRef;
use x509_parser::{certificate::X509Certificate, parse_x509_certificate, time::ASN1Time};
pub struct StoredCertificate {
    pub fingerprint: String,
    pub algorithm: String,
}
pub struct CertificateMap {
    pub stored: HashMap<String, StoredCertificate>,
    path_to_file: PathBuf,
}

impl CertificateMap {
    pub fn new(path: &Path) -> Self {
        Self {
            stored: HashMap::new(),
            path_to_file: path.to_owned(),
        }
    }
    fn verify(
        &mut self,
        hostname: &str,
        der_encoded_cert: &[u8],
        der_parsed_cert: &X509Certificate,
    ) -> Result<ServerCertVerified, TLSError> {
        let hostname = hostname.to_lowercase();
        if !Self::verify_expiry(&hostname, der_parsed_cert) {
            return Err(TLSError::WebPKIError(webpki::Error::CertExpired));
        }

        // TODO: check against certificate we've saved if they exists for this hostname.

        // TODO: get fingerprint and insert certificate into our map to save otherwise.
        let fingerprint = ring::digest::digest(&digest::SHA256, der_encoded_cert);
        Ok(ServerCertVerified::assertion())
    }
    fn verify_expiry(_hostname: &str, cert: &X509Certificate) -> bool {
        if !cert.validity().is_valid_at(ASN1Time::now()) {
            return false;
        }
        cert.verify_signature(None).is_ok()
    }
}
pub struct TOFUVerifier {
    pub map: Arc<Mutex<CertificateMap>>,
}

impl TOFUVerifier {
    pub fn new(path: &Path) -> Self {
        Self {
            map: Arc::new(Mutex::new(CertificateMap::new(path))),
        }
    }
}

impl ServerCertVerifier for TOFUVerifier {
    fn verify_server_cert(
        &self,
        _: &RootCertStore,
        presented_certs: &[Certificate],
        hostname: DNSNameRef,
        _: &[u8],
    ) -> Result<ServerCertVerified, TLSError> {
        let en_cert = &presented_certs[0].0;
        let (_, cert) = parse_x509_certificate(en_cert)
            .map_err(|_| TLSError::WebPKIError(webpki::Error::BadDER))?;
        self.map
            .lock()
            .map_err(|_| TLSError::General("Unable to Lock!".to_string()))?
            .verify(AsRef::<str>::as_ref(&hostname.to_owned()), en_cert, &cert)
    }
}
