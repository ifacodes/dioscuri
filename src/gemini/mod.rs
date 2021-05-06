mod gemtext;
mod response;

use gemtext::GemText;
use tui::text::{Span, Spans};

pub struct Page {
    contents: Vec<GemText>,
}

impl<'a> Page {
    pub fn format_page(&self) -> Spans<'a> {
        let mut formatted_lines: Vec<Span<'a>> = Vec::new();
        let mut pre_flag: bool = false;

        for line in &self.contents {
            match line {
                GemText::Pre => pre_flag = !pre_flag,
                GemText::Text(s) if !pre_flag => formatted_lines.push(Span::raw(s.to_owned())),
                GemText::Heading(s) => {
                    formatted_lines.push(Span::raw(s[1..s.len()].trim().to_owned()))
                }
                GemText::List(s) => formatted_lines.push(Span::raw(format!(
                    "    {}",
                    s[1..s.len()].trim().to_owned()
                ))),
                GemText::Quote(s) => formatted_lines.push(Span::raw(s.to_owned())),
                _ => formatted_lines.push(Span::raw("")),
            }
        }

        Spans::from(formatted_lines)
    }
}
