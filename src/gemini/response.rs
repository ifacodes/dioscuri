use super::Page;
use anyhow::{anyhow, Result};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Status {
    Input,
    SensitiveInput,

    Success,
    TemporaryRedirect,
    PermanantRedirect,

    TemporaryFailure,
    ServerUnavailable,
    CGIError,
    ProxyError,
    SlowDown,

    PermanentFailure,
    NotFound,
    Gone,
    ProxyRequestRefused,
    BadRequest,

    ClientCertificateRequried,
    CertificateNotAuthorised,
    CertificateNotValid,
}

impl FromStr for Status {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "10" => Ok(Status::Input),
            "11" => Ok(Status::SensitiveInput),
            "20" => Ok(Status::Success),
            "30" => Ok(Status::TemporaryRedirect),
            "31" => Ok(Status::PermanantRedirect),
            "40" => Ok(Status::TemporaryFailure),
            "41" => Ok(Status::ServerUnavailable),
            "42" => Ok(Status::CGIError),
            "43" => Ok(Status::ProxyError),
            "44" => Ok(Status::SlowDown),
            "50" => Ok(Status::PermanentFailure),
            "51" => Ok(Status::NotFound),
            "52" => Ok(Status::Gone),
            "53" => Ok(Status::ProxyRequestRefused),
            "59" => Ok(Status::BadRequest),
            "60" => Ok(Status::ClientCertificateRequried),
            "61" => Ok(Status::CertificateNotAuthorised),
            "62" => Ok(Status::CertificateNotValid),
            _ => Err(anyhow!("Not a valid status sode: {}", s)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ResponseHeader {
    pub status: Status,
    pub meta: String,
}

impl FromStr for ResponseHeader {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let header: Vec<&str> = s.trim().splitn(2, ' ').collect();

        let status = header
            .get(0)
            .ok_or(anyhow!("Response Header is missing status sode!"))?
            .parse()?;
        let meta = header
            .get(1)
            .ok_or(anyhow!("Response Header is missing meta!"))?;

        Ok(ResponseHeader {
            status,
            meta: meta.to_string(),
        })
    }
}

pub struct Response {
    pub header: ResponseHeader,
    pub page: Page,
}
