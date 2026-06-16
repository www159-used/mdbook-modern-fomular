//! KaTeX HTML rendering.

use katex::Opts;
use log::warn;

use crate::delimiter::Delimiter;

pub fn render_math(
    expression: &str,
    opts: &Opts,
    delimiter: &Delimiter,
    include_src: bool,
) -> String {
    match katex::render_with_opts(expression, opts) {
        Ok(html) => wrap_output(expression, &html.replace('\n', " "), include_src),
        Err(err) => {
            warn!("KaTeX render failed, keeping original expression: {err}");
            delimiter.wrap(expression)
        }
    }
}

fn wrap_output(expression: &str, html: &str, include_src: bool) -> String {
    if !include_src {
        return html.to_owned();
    }

    let escaped = expression.replace('"', r#"\""#).replace('\n', "&#10;");
    format!(r#"<data class="katex-src" value="{escaped}">{html}</data>"#)
}
