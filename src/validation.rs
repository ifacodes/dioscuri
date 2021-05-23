use anyhow::Result;
use data_encoding::HEXUPPER;
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
            if let Some(dir) = self.path_to_file.parent() {
                std::fs::create_dir_all(dir).unwrap();
            }
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

        let fingerprint =
            HEXUPPER.encode(ring::digest::digest(&digest::SHA256, der_encoded_cert).as_ref());
        CertificateMap::match_certificate(self, hostname.as_str(), fingerprint, der_parsed_cert);

        CertificateMap::save_to_file(self);

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
        new_fingerprint: String,
        new_cert: &X509Certificate,
    ) {
        if let Some(cert) = self.stored_certificates.get_mut(hostname) {
            if new_fingerprint == cert.fingerprint {
                return;
            }
            // if not, is the stored certificate expired?
            if ASN1Time::now() < ASN1Time::from_timestamp(cert.not_after) {
                // TODO: show the user a warning! this certificate might be dodgy
                println!("Certificate Might Be Dodgy!");
            } else {
                // update stored cert with new certificate
                *cert = StoredCertificate {
                    fingerprint: new_fingerprint.to_owned(),
                    not_after: new_cert.validity().not_after.timestamp(),
                };
            }
            return;
        }
        self.stored_certificates.insert(
            hostname.to_lowercase(),
            StoredCertificate {
                fingerprint: new_fingerprint.to_owned(),
                not_after: new_cert.validity().not_after.timestamp(),
            },
        );
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
