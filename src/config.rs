//! Preprocessor configuration loaded from `book.toml`.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use log::warn;
use mdbook_preprocessor::config::Config as BookConfig;
use serde::Deserialize;

use crate::constants::CONFIG_TABLE;
use crate::delimiter::Delimiter;

/// Per-chapter rendering options derived from [`Config`].
#[derive(Clone, Debug)]
pub struct RenderOptions {
    pub include_src: bool,
    pub display_delimiter: Delimiter,
    pub inline_delimiter: Delimiter,
}

/// Full preprocessor configuration.
#[derive(Debug, Clone, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Config {
    pub output: String,
    pub leqno: bool,
    pub fleqn: bool,
    pub throw_on_error: bool,
    pub error_color: String,
    pub min_rule_thickness: f64,
    pub max_size: f64,
    pub max_expand: i32,
    pub trust: bool,
    pub no_css: bool,
    pub include_src: bool,
    pub macros: Option<String>,
    pub block_delimiter: Delimiter,
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
    pub fn render_options(&self) -> RenderOptions {
        RenderOptions {
            include_src: self.include_src,
            display_delimiter: self.block_delimiter.clone(),
            inline_delimiter: self.inline_delimiter.clone(),
        }
    }

    pub fn macro_path<P: AsRef<Path>>(&self, book_root: P) -> Option<PathBuf> {
        macro_path(book_root, &self.macros)
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
