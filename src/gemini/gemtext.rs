use anyhow::Result;
use std::str::FromStr;

pub struct Link {
    pub url: String,
    pub link_name: String,
}
pub enum GemText {
    Text(String),
    Link(String),
    Pre,
    Heading(String),
    List(String),
    Quote(String),
}

impl FromStr for GemText {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            s if s.starts_with("```") => Ok(GemText::Pre),
            s if s.starts_with("=> ") => Ok(GemText::Link(s.to_owned())),
            s if s.starts_with("# ") => Ok(GemText::Heading(s.to_owned())),
            s if s.starts_with("> ") => Ok(GemText::Quote(s.to_owned())),
            s if s.starts_with("* ") => Ok(GemText::List(s.to_owned())),
            s => Ok(GemText::Text(s.to_owned())),
        }
    }
}
