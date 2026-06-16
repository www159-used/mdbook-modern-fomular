//! mdBook preprocessor for rendering LaTeX math with KaTeX.

pub mod chapter;
pub mod config;
pub mod constants;
pub mod delimiter;
pub mod katex;
pub mod preprocessor;
pub mod scan;

pub use config::Config;
pub use preprocessor::FomularPreprocessor;

/// Initialize `log` output to stderr. Safe to call more than once.
pub fn init_logger() {
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
        .try_init();
}
