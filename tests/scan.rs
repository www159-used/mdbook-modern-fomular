use mdbook_modern_fomular::chapter::{self, Segment};
use mdbook_modern_fomular::Config;

#[test]
fn plain_text_is_one_segment() {
    let options = Config::default().render_options();
    let segs = chapter::segments("hello world", &options);
    assert_eq!(segs.len(), 1);
    assert!(matches!(segs[0], Segment::Text("hello world")));
}

#[test]
fn ignores_math_inside_inline_code() {
    let options = Config::default().render_options();
    let segs = chapter::segments(r"`$x$`", &options);
    assert_eq!(segs.len(), 1);
    assert!(matches!(segs[0], Segment::Text(r"`$x$`")));
}

#[test]
fn ignores_math_inside_fenced_code_block() {
    let options = Config::default().render_options();
    let raw = r"``` $\omega$ ```";
    let segs = chapter::segments(raw, &options);
    assert_eq!(segs.len(), 1);
    assert_eq!(segs[0], Segment::Text(raw));
}

#[test]
fn preserves_escaped_dollar_signs() {
    let raw = r"Some text, \$\$ and more text.";
    let options = Config::default().render_options();
    let segs = chapter::segments(raw, &options);
    assert_eq!(segs.len(), 1);
    assert_eq!(segs[0], Segment::Text(raw));
}
