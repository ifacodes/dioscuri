pub enum InputMode {
    UrlInput,
    NoInput,
}

pub struct App {
    pub url: String,
    pub input_mode: InputMode,
    pub previous_urls: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        App {
            url: "gemini://gemini.circumlunar.space/".to_owned(),
            input_mode: InputMode::NoInput,
            previous_urls: Vec::new(),
        }
    }
}
