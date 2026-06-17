#![allow(dead_code)]

use std::collections::HashMap;

use mdbook_modern_fomular::chapter::RenderSetup;
use mdbook_modern_fomular::Config;

pub fn output_for(raw: &str) -> (String, String) {
    output_with(raw, &Config::default(), HashMap::new())
}

pub fn output_with(
    raw: &str,
    config: &Config,
    macros: HashMap<String, String>,
) -> (String, String) {
    let (css, mut rendered) = outputs_with(&[raw], macros, config);
    (css, rendered.pop().expect("one chapter"))
}

pub fn outputs_with(
    raws: &[&str],
    macros: HashMap<String, String>,
    config: &Config,
) -> (String, Vec<String>) {
    let setup = RenderSetup::from_macros(config, macros);
    let css = setup.css().to_owned();
    let contents = raws.iter().map(|raw| setup.render(raw)).collect();
    (css, contents)
}

/// Compare KaTeX HTML while ignoring `style="..."` attributes.
pub fn assert_same_katex_html(left: &str, right: &str) {
    assert_eq!(strip_style_attrs(left), strip_style_attrs(right));
}

fn strip_style_attrs(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(start) = rest.find("style=\"") {
        out.push_str(&rest[..start]);
        rest = &rest[start + 7..];
        if let Some(end) = rest.find('"') {
            rest = &rest[end + 1..];
        } else {
            break;
        }
    }
    out.push_str(rest);
    out
}
