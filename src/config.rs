//! Preprocessor configuration loaded from `book.toml`.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use mdbook_preprocessor::config::Config as BookConfig;
use log::warn;
use serde::Deserialize;

use crate::constants::CONFIG_TABLE;
use crate::delimiter::Delimiter;

/// Per-chapter rendering options derived from [`Config`].
#[derive(Clone, Debug)]
pub struct RenderOptions {
    /// Include original math source in rendered HTML.
    pub include_src: bool,
    /// Display math delimiters.
    pub display_delimiter: Delimiter,
    /// Inline math delimiters.
    pub inline_delimiter: Delimiter,
}

/// Full preprocessor configuration.
#[derive(Debug, Clone, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Config {
    /// KaTeX output format.
    pub output: String,
    /// Render tags on the left.
    pub leqno: bool,
    /// Flush display math left.
    pub fleqn: bool,
    /// Fail on invalid LaTeX.
    pub throw_on_error: bool,
    /// Color for parse errors.
    pub error_color: String,
    /// Minimum rule thickness in ems.
    pub min_rule_thickness: f64,
    /// Maximum user-specified size.
    pub max_size: f64,
    /// Maximum macro expansion count.
    pub max_expand: i32,
    /// Trust user input.
    pub trust: bool,
    /// Skip KaTeX stylesheet injection.
    pub no_css: bool,
    /// Include math source in output.
    pub include_src: bool,
    /// Path to macro definitions.
    pub macros: Option<String>,
    /// Display math delimiters.
    pub block_delimiter: Delimiter,
    /// Inline math delimiters.
    pub inline_delimiter: Delimiter,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            output: "html".into(),
            leqno: false,
            fleqn: false,
            throw_on_error: true,
            error_color: "#cc0000".into(),
            min_rule_thickness: -1.0,
            max_size: f64::INFINITY,
            max_expand: 1000,
            trust: false,
            no_css: false,
            include_src: false,
            macros: None,
            block_delimiter: Delimiter::symmetric("$$"),
            inline_delimiter: Delimiter::symmetric("$"),
        }
    }
}

impl Config {
    /// Build per-chapter options from book configuration.
    pub fn render_options(&self) -> RenderOptions {
        RenderOptions {
            include_src: self.include_src,
            display_delimiter: self.block_delimiter.clone(),
            inline_delimiter: self.inline_delimiter.clone(),
        }
    }

    /// Resolve the absolute path to the macro file, if configured.
    pub fn macro_path<P: AsRef<Path>>(&self, book_root: P) -> Option<PathBuf> {
        macro_path(book_root, &self.macros)
    }

    /// Build inline and display KaTeX option sets.
    pub fn katex_opts<P: AsRef<Path>>(&self, book_root: P) -> (katex::Opts, katex::Opts) {
        let macros = load_macros(book_root, &self.macros);
        self.katex_opts_from_macros(macros)
    }

    /// Build KaTeX option sets from an in-memory macro table.
    pub fn katex_opts_from_macros(
        &self,
        macros: std::collections::HashMap<String, String>,
    ) -> (katex::Opts, katex::Opts) {
        let output_type = match self.output.as_str() {
            "html" => katex::OutputType::Html,
            "mathml" => katex::OutputType::Mathml,
            "htmlAndMathml" => katex::OutputType::HtmlAndMathml,
            other => {
                warn!(
                    "[{CONFIG_TABLE}] invalid output `{other}`; valid values: html, mathml, htmlAndMathml"
                );
                katex::OutputType::Html
            }
        };

        let mut builder = katex::Opts::builder();
        builder
            .output_type(output_type)
            .leqno(self.leqno)
            .fleqn(self.fleqn)
            .throw_on_error(self.throw_on_error)
            .error_color(self.error_color.clone())
            .macros(macros)
            .min_rule_thickness(self.min_rule_thickness)
            .max_size(self.max_size)
            .max_expand(self.max_expand)
            .trust(self.trust);

        let inline = builder
            .clone()
            .display_mode(false)
            .build()
            .expect("valid inline KaTeX options");
        let display = builder
            .display_mode(true)
            .build()
            .expect("valid display KaTeX options");
        (inline, display)
    }
}

/// Resolve the absolute path to a macro file relative to the book root.
pub fn macro_path<P: AsRef<Path>>(book_root: P, macros: &Option<String>) -> Option<PathBuf> {
    macros
        .as_ref()
        .map(|path| book_root.as_ref().join(path))
}

/// Load preprocessor config from mdBook's book configuration.
pub fn load(book_cfg: &BookConfig) -> Config {
    match book_cfg
        .get::<toml::Value>(CONFIG_TABLE)
        .unwrap_or_default()
    {
        Some(raw) => raw.try_into().unwrap_or_default(),
        None => Config::default(),
    }
}

/// Parse macro definitions from file content.
pub fn parse_macros(content: &str) -> HashMap<String, String> {
    content
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if !line.starts_with('\\') {
                return None;
            }
            let (name, value) = line.split_once(':')?;
            Some((name.to_owned(), value.to_owned()))
        })
        .collect()
}

/// Load macro definitions from disk.
pub fn load_macros<P: AsRef<Path>>(
    book_root: P,
    macros: &Option<String>,
) -> HashMap<String, String> {
    let Some(path) = macro_path(book_root, macros) else {
        return HashMap::new();
    };

    match fs::read_to_string(&path) {
        Ok(content) => parse_macros(&content),
        Err(err) => {
            warn!("failed to read macros from {}: {err}", path.display());
            HashMap::new()
        }
    }
}
