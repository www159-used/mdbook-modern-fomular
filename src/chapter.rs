//! Render math within a single chapter.

use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;

use crate::config::{Config, RenderOptions};
use crate::constants::KATEX_STYLESHEET;
use crate::katex::render_math;
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
    inline_opts: katex::Opts,
    display_opts: katex::Opts,
}

impl RenderSetup {
    pub fn from_config<P: AsRef<Path>>(config: &Config, book_root: P) -> Self {
        Self::build(config, config.katex_opts(book_root))
    }

    pub fn from_macros(config: &Config, macros: HashMap<String, String>) -> Self {
        Self::build(config, config.katex_opts_from_macros(macros))
    }

    fn build(config: &Config, opts: (katex::Opts, katex::Opts)) -> Self {
        Self {
            options: config.render_options(),
            css: stylesheet(config),
            inline_opts: opts.0,
            display_opts: opts.1,
        }
    }

    pub fn css(&self) -> &'static str {
        self.css
    }

    pub fn render(&self, source: &str) -> String {
        render(
            source,
            &self.options,
            self.css,
            &self.inline_opts,
            &self.display_opts,
        )
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

pub fn render(
    source: &str,
    options: &RenderOptions,
    css: &str,
    inline_opts: &katex::Opts,
    display_opts: &katex::Opts,
) -> String {
    let mut output = String::from(css);
    output.push_str(
        &segments(source, options)
            .into_iter()
            .map(|segment| match segment {
                Segment::Text(text) => Cow::Borrowed(text),
                Segment::Inline(expr) => Cow::Owned(render_math(
                    expr,
                    inline_opts,
                    &options.inline_delimiter,
                    options.include_src,
                )),
                Segment::Display(expr) => Cow::Owned(render_math(
                    expr,
                    display_opts,
                    &options.display_delimiter,
                    options.include_src,
                )),
            })
            .collect::<String>(),
    );
    output
}

pub fn stylesheet(config: &Config) -> &'static str {
    if config.no_css {
        ""
    } else {
        KATEX_STYLESHEET
    }
}
