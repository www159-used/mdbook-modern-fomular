mod common;

use std::collections::HashMap;

use mdbook_modern_fomular::Config;

use common::{output_for, outputs_with};

#[test]
fn dollar_escaping_inside_expr() {
    let raw_content = r"We randomly assign: $r \xleftarrow{\$} G $.";
    let (stylesheet_header, rendered_content) = output_for(raw_content);
    let body = rendered_content.strip_prefix(&stylesheet_header).unwrap();
    assert!(body.starts_with("We randomly assign: "));
    assert!(body.contains("class=\"katex\""));
    assert!(body.contains("x-arrow"));
    assert!(body.ends_with('.'));
}

#[test]
fn inline_rendering() {
    let (stylesheet_header, rendered_content) =
        output_for(r"Some text, $\nabla f(x) \in \mathbb{R}^n$, and more text.");
    let body = rendered_content.strip_prefix(&stylesheet_header).unwrap();
    assert!(body.contains("Some text, "));
    assert!(body.contains("class=\"katex\""));
    assert!(body.contains('∇'));
    assert!(body.contains('∈'));
    assert!(body.contains("mathbb"));
    assert!(body.ends_with(", and more text."));
}

#[test]
fn display_rendering() {
    let (stylesheet_header, rendered_content) =
        output_for(r"Block formula: $$\nabla f(x) \in \mathbb{R}^n$$");
    assert!(rendered_content.contains("katex-display"));
    assert!(rendered_content.starts_with(&stylesheet_header));
}

#[test]
fn invalid_expr_inline() {
    let raw_content = r"$\<$";
    let (stylesheet_header, rendered_content) = output_for(raw_content);
    assert_eq!(stylesheet_header + raw_content, rendered_content);
}

#[test]
fn invalid_expr_display() {
    let raw_content = r"$$ \< $$";
    let (stylesheet_header, rendered_content) = output_for(raw_content);
    assert_eq!(stylesheet_header + raw_content, rendered_content);
}

#[test]
fn escaping_backtick() {
    let raw_content = r"\`$\omega$\`";
    let (stylesheet_header, rendered_content) = output_for(raw_content);
    let body = rendered_content.strip_prefix(&stylesheet_header).unwrap();
    assert!(body.starts_with("\\`"));
    assert!(body.ends_with("\\`"));
    assert!(body.contains("class=\"katex\""));
    assert!(body.contains('ω'));
}

#[test]
fn include_src() {
    let raw_content = r"Define $f(x)$:

$$
f(x)=x^2\\

x\in\R
$$";
    let cfg = Config {
            include_src: true,
            ..Config::default()
        };
    let (stylesheet_header, rendered_content) = outputs_with(
        &[raw_content],
        HashMap::new(),
        &cfg,
    );
    let body = rendered_content[0].strip_prefix(&stylesheet_header).unwrap();
    assert!(body.contains(r#"class="katex-src" value="f(x)""#));
    assert!(body.contains("katex-display"));
    assert!(body.contains(r#"value="&#10;f(x)=x^2\\&#10;&#10;x\in\R&#10;""#));
}

#[test]
fn renders_vmatrix() {
    let raw = r"$$\begin{vmatrix}a&b\\c&d\end{vmatrix}$$";
    let (stylesheet, rendered) = output_for(raw);
    assert!(rendered.contains("katex-display"));
    assert!(rendered.starts_with(&stylesheet));
}
