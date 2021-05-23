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
        Self {
            url: "gemini://gemini.circumlunar.space/".to_owned(),
            input_mode: InputMode::NoInput,
            previous_urls: Vec::new(),
        }
    }
}

impl App {
    pub fn update_url(&mut self, new_url: String) {
        self.url = new_url;
    }
}
