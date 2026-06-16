mod common;

use std::collections::HashMap;
use std::path::PathBuf;

use mdbook_modern_fomular::config;
use mdbook_modern_fomular::Config;

use common::outputs_with;

#[test]
fn macro_file_loading() {
    let cfg_str = r#"
    [book]
    src = "src"

    [preprocessor.modern-fomular]
    macros = "macros.txt"
    "#;

    let book_cfg = cfg_str.parse().unwrap();
    let cfg = config::load(&book_cfg);

    assert_eq!(
        cfg.macro_path(PathBuf::from("book")),
        Some(PathBuf::from("book/macros.txt"))
    );
}

#[test]
fn macros_without_argument() {
    let mut macros = HashMap::new();
    macros.insert(String::from(r"\grad"), String::from(r"\nabla"));
    let raw_content_no_macro = r"Some text, $\nabla f(x) \in \mathbb{R}^n$, and more text.";
    let raw_content_macro = r"Some text, $\grad f(x) \in \mathbb{R}^n$, and more text.";
    let (_, rendered) = outputs_with(
        &[raw_content_macro, raw_content_no_macro],
        macros,
        Config::default(),
    );
    assert_eq!(rendered[0], rendered[1]);
}

#[test]
fn macros_with_argument() {
    let mut macros = HashMap::new();
    macros.insert(String::from(r"\R"), String::from(r"\mathbb{R}^#1"));
    let raw_content_no_macro = r"Some text, $\nabla f(x) \in \mathbb{R}^1$, and more text.";
    let raw_content_macro = r"Some text, $\nabla f(x) \in \R{1}$, and more text.";
    let (_, rendered) = outputs_with(
        &[raw_content_macro, raw_content_no_macro],
        macros,
        Config::default(),
    );
    assert_eq!(rendered[0], rendered[1]);
}
