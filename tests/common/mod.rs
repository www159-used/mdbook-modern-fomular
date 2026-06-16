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
    let setup = RenderSetup::from_macros(config, macros);
    let css = setup.css().to_owned();
    let content = setup.render(raw);
    (css, content)
}

pub fn outputs_with(
    raws: &[&str],
    macros: HashMap<String, String>,
    config: Config,
) -> (String, Vec<String>) {
    let setup = RenderSetup::from_macros(&config, macros);
    let css = setup.css().to_owned();
    let contents = raws.iter().map(|raw| setup.render(raw)).collect();
    (css, contents)
}
