//! Render math within a single chapter.

use std::collections::HashMap;
use std::path::Path;

use crate::config::{self, Config, RenderOptions};
use crate::constants::KATEX_STYLESHEET;
use crate::katex::Engine;
use crate::scan::{ScanEvent, Scanner};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Segment<'source> {
    Text(&'source str),
    Inline(&'source str),
    Display(&'source str),
}

/// Prepared render state shared across chapters in one book.
pub struct RenderSetup {
    options: RenderOptions,
    css: &'static str,
    engine: Engine,
}

impl RenderSetup {
    pub fn from_config<P: AsRef<Path>>(config: &Config, book_root: P) -> Self {
        let macros = config::load_macros(book_root, &config.macros);
        Self::from_macros(config, macros)
    }

    pub fn from_macros(config: &Config, macros: HashMap<String, String>) -> Self {
        Self {
            options: config.render_options(),
            css: stylesheet(config),
            engine: Engine::from_macros(config, &macros),
        }
    }

    pub fn css(&self) -> &'static str {
        self.css
    }

    pub fn render(&self, source: &str) -> String {
        let mut output = String::from(self.css);
        for segment in segments(source, &self.options) {
            match segment {
                Segment::Text(text) => output.push_str(text),
                Segment::Inline(expr) => output.push_str(&self.engine.render(
                    expr,
                    false,
                    &self.options.inline_delimiter,
                    self.options.include_src,
                )),
                Segment::Display(expr) => output.push_str(&self.engine.render(
                    expr,
                    true,
                    &self.options.display_delimiter,
                    self.options.include_src,
                )),
            }
        }
        output
    }
}

pub fn segments<'source>(source: &'source str, options: &RenderOptions) -> Vec<Segment<'source>> {
    let mut out = Vec::new();
    let scanner = Scanner::new(
        source,
        &options.display_delimiter,
        &options.inline_delimiter,
    );

    let mut at = 0;
    for event in scanner {
        match event {
            ScanEvent::SegmentStart(start) => at = start,
            ScanEvent::TextEnd(end) => out.push(Segment::Text(&source[at..end])),
            ScanEvent::InlineEnd(end) => {
                out.push(Segment::Inline(&source[at..end]));
                at = end;
            }
            ScanEvent::DisplayEnd(end) => {
                out.push(Segment::Display(&source[at..end]));
                at = end;
            }
        }
    }

    if at < source.len() {
        out.push(Segment::Text(&source[at..]));
    }

    out
}

fn stylesheet(config: &Config) -> &'static str {
    if config.no_css {
        ""
    } else {
        KATEX_STYLESHEET
    }
}
