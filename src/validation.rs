use webpki::DNSNameRef;
use x509_parser::parse_x509_certificate;

pub struct TOFUVerifier {}

impl TOFUVerifier {
    fn new() -> Arc<dyn ServerCertVerifier> {
        Arc::new(TOFUVerifier {});
    }
}

impl ServerCertVerifier for TOFUVerifier {
    fn verify_server_cert(
        &self,
        _: &RootCertStore,
        presented_certs: &[Certificate],
        dns_name: DNSNameRef,
        _: &[u8],
    ) -> Result<ServerCertVerified, TLSError> {
        for cert in presented_certs.into_iter() {
            let result = parse_x509_certificate(cert);
            match result {
                Ok((rem, cert)) => {
                    assert!(rem.is_empty());
                    let subject = &cert.tbs_certificate.subject;
                    let issuer = &cert.tbs_certificate.issuer;
                    let valid = &cert.tbs_certificate.validity.not_after;
                    println!("Subject: {}", subject);
                    println!("Issuer: {}", issuer, valid);
                }
                _ => panic!("parsing failed! {:?}", res),
            }
        }
    }
}
