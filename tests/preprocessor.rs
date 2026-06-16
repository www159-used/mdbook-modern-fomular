use mdbook_preprocessor::Preprocessor;
use mdbook_modern_fomular::constants::PREPROCESSOR_NAME;
use mdbook_modern_fomular::FomularPreprocessor;

#[test]
fn name_matches_mdbook_convention() {
    let pre = FomularPreprocessor;
    let preprocessor: &dyn Preprocessor = &pre;
    assert_eq!(preprocessor.name(), PREPROCESSOR_NAME);
}

#[test]
fn supports_html_renderer() {
    let pre = FomularPreprocessor;
    assert!(pre.supports_renderer("html").unwrap());
}
