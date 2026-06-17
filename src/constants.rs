//! Shared constants for the preprocessor.

/// mdBook table name and CLI binary suffix.
pub const PREPROCESSOR_NAME: &str = "modern-fomular";

/// Config table in `book.toml`.
pub const CONFIG_TABLE: &str = "preprocessor.modern-fomular";

macro_rules! define_katex_assets {
    ($version:literal) => {
        /// KaTeX release tracked by `katex-rs` 0.2.4
        /// (commit 785315c0f630f65347cac14b3ec72629cfe7631e).
        pub const KATEX_VERSION: &str = $version;

        /// Default KaTeX stylesheet injected into each chapter.
        pub const KATEX_STYLESHEET: &str = concat!(
            r#"<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@"#,
            $version,
            r#"/dist/katex.min.css">"#,
            "\n\n",
        );
    };
}

define_katex_assets!("0.16.25");
