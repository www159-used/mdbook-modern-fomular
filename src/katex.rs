//! KaTeX rendering via [katex-rs](https://github.com/katex-rs/katex-rs).

use std::collections::HashMap;

use katex::macro_expander::MacroMap;
use katex::macros::MacroDefinition;
use katex::{render_to_string, KatexContext, OutputFormat, Settings, TrustSetting};
use log::warn;

use crate::config::Config;
use crate::constants::CONFIG_TABLE;
use crate::delimiter::Delimiter;

/// Shared KaTeX state for one book build.
pub struct Engine {
    ctx: KatexContext,
    inline: Settings,
    display: Settings,
}

impl Engine {
    pub fn from_macros(config: &Config, macros: &HashMap<String, String>) -> Self {
        let macro_map = macros_to_map(macros);
        let inline = build_settings(config, false, macro_map);
        let mut display = inline.clone();
        display.display_mode = true;
        Self {
            ctx: KatexContext::default(),
            inline,
            display,
        }
    }

    pub fn render(
        &self,
        expression: &str,
        display: bool,
        delimiter: &Delimiter,
        include_src: bool,
    ) -> String {
        let settings = if display {
            &self.display
        } else {
            &self.inline
        };
        render_math(&self.ctx, expression, settings, delimiter, include_src)
    }
}

pub fn render_math(
    ctx: &KatexContext,
    expression: &str,
    settings: &Settings,
    delimiter: &Delimiter,
    include_src: bool,
) -> String {
    match render_to_string(ctx, expression, settings) {
        Ok(html) => wrap_output(expression, html, include_src),
        Err(err) => {
            warn!("KaTeX render failed, keeping original expression: {err}");
            delimiter.wrap(expression)
        }
    }
}

fn wrap_output(expression: &str, mut html: String, include_src: bool) -> String {
    if html.contains('\n') {
        html = html.replace('\n', " ");
    }

    if !include_src {
        return html;
    }

    let escaped = expression.replace('"', r#"\""#).replace('\n', "&#10;");
    format!(r#"<data class="katex-src" value="{escaped}">{html}</data>"#)
}

fn build_settings(config: &Config, display_mode: bool, macros: MacroMap) -> Settings {
    Settings::builder()
        .display_mode(display_mode)
        .output(output_format(config))
        .leqno(config.leqno)
        .fleqn(config.fleqn)
        .throw_on_error(config.throw_on_error)
        .error_color(config.error_color.clone())
        .macros(macros)
        .min_rule_thickness(config.min_rule_thickness)
        .max_size(config.max_size)
        .max_expand(config.max_expand.max(0) as usize)
        .trust(TrustSetting::Bool(config.trust))
        .build()
}

fn output_format(config: &Config) -> OutputFormat {
    match config.output.as_str() {
        "html" => OutputFormat::Html,
        "mathml" => OutputFormat::Mathml,
        "htmlAndMathml" => OutputFormat::HtmlAndMathml,
        other => {
            warn!(
                "[{CONFIG_TABLE}] invalid output `{other}`; valid values: html, mathml, htmlAndMathml"
            );
            OutputFormat::Html
        }
    }
}

fn macros_to_map(macros: &HashMap<String, String>) -> MacroMap {
    let mut map = MacroMap::default();
    for (name, value) in macros {
        map.insert(name.clone(), MacroDefinition::String(value.clone()));
    }
    map
}
