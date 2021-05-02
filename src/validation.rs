use anyhow::Result;
use ring::digest;
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
    pub not_after: i64,
}
#[derive(Default)]
pub struct CertificateMap {
    pub stored_certificates: HashMap<String, StoredCertificate>,
    path_to_file: PathBuf,
}

impl CertificateMap {
    pub fn new(path: &Path) -> Self {
        if path.exists() {
            CertificateMap::load_from_file(path);
        }
        Self {
            stored_certificates: HashMap::new(),
            path_to_file: path.to_owned(),
        }
    }
    fn load_from_file(path: &Path) -> Self {
        let file = std::fs::read_to_string(path).expect("File could not be loaded!");
        let mut map = HashMap::new();
        for line in file.lines() {
            let values = line.splitn(3, ';').collect::<Vec<&str>>();
            let (host, fingerprint, not_after) = (
                values[0].to_string(),
                values[1].to_string(),
                values[2]
                    .parse::<i64>()
                    .expect("unable to parse the expiry time of saved certificate!"),
            );
            map.insert(
                host.to_lowercase(),
                StoredCertificate {
                    fingerprint,
                    not_after,
                },
            );
        }
        return Self {
            stored_certificates: map,
            path_to_file: path.to_owned(),
        };
    }
    fn save_to_file(&self) {
        let mut contents = String::new();
        for (host, cert) in &self.stored_certificates {
            contents.push_str(&format!(
                "{};{};{}\n",
                host, cert.fingerprint, cert.not_after
            ));
        }
        std::fs::write(&self.path_to_file, contents).expect("unable to save certificates to store!")
    }
    fn verify(
        &mut self,
        hostname: &str,
        der_encoded_cert: &[u8],
        der_parsed_cert: &X509Certificate,
    ) -> Result<ServerCertVerified, TLSError> {
        let hostname = hostname.to_lowercase();
        CertificateMap::verify_expiry(der_parsed_cert)?;
        der_parsed_cert
            .verify_signature(None)
            .map_err(|_| TLSError::WebPKIError(webpki::Error::InvalidSignatureForPublicKey))?;

        // TODO: check against certificate we've saved if they exists for this hostname.

        // TODO: get fingerprint and insert certificate into our map to save otherwise.
        let fingerprint = ring::digest::digest(&digest::SHA256, der_encoded_cert);
        Ok(ServerCertVerified::assertion())
    }
    fn verify_expiry(parsed_cert: &X509Certificate) -> Result<(), TLSError> {
        if !parsed_cert.validity().is_valid_at(ASN1Time::now()) {
            return Err(TLSError::WebPKIError(webpki::Error::CertExpired));
        }
        Ok(())
    }
    fn match_certificate(
        &mut self,
        hostname: &str,
        new_fingerprint: &[u8],
        new_cert: &X509Certificate,
    ) {
        if let Some(cert) = self.stored_certificates.get_mut(hostname) {
            // TODO: invert conditions!!
            // unwrap because it will always be valid
            if std::str::from_utf8(new_fingerprint).unwrap() != cert.fingerprint {
                // if not, is the stored certificate expired?
                if ASN1Time::now() >= ASN1Time::from_timestamp(cert.not_after) {
                    // update stored cert with new certificate
                    *cert = StoredCertificate {
                        fingerprint: std::str::from_utf8(new_fingerprint).unwrap().to_string(),
                        not_after: new_cert.validity().not_after.timestamp(),
                    };
                } else {
                    // TODO: show the user a warning! this certificate might be dodgy
                }
            }
        }
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
            .map_err(|_| TLSError::General("unable to lock certificate map mutex!".to_string()))?
            .verify(AsRef::<str>::as_ref(&hostname.to_owned()), en_cert, &cert)
    }
}
