//! Shared constants for the preprocessor.

/// mdBook table name and CLI binary suffix.
pub const PREPROCESSOR_NAME: &str = "modern-fomular";

/// Config table in `book.toml`.
pub const CONFIG_TABLE: &str = "preprocessor.modern-fomular";

/// Supported mdBook semver range.
pub const MDBOOK_COMPAT: &str = "^0.5";

/// Default KaTeX stylesheet injected into each chapter.
pub const KATEX_STYLESHEET: &str = r#"<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.4/dist/katex.min.css">

"#;
