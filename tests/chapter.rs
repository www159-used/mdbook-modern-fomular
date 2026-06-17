mod common;

use std::collections::HashMap;

use mdbook_modern_fomular::delimiter::Delimiter;
use mdbook_modern_fomular::Config;

use common::{output_for, outputs_with};

#[test]
fn rendering_table_with_math() {
    let raw_content = r"| Syntax | Description |
| --- | ----------- |
| $\vec{a}$ | Title |
| Paragraph | Text |";
    let (stylesheet_header, rendered_content) = output_for(raw_content);
    let expected_output = stylesheet_header + raw_content;
    assert_eq!(
        expected_output.lines().count(),
        rendered_content.lines().count()
    );
}

#[test]
fn rendering_delimiter_in_inline_code_when_inline_delimiter_starts_with_backtick() {
    let raw_content = r"`` `$\omega$` ``";
    let cfg = Config {
        inline_delimiter: Delimiter {
            left: "`$".into(),
            right: "$`".into(),
        },
        ..Config::default()
    };
    let (stylesheet_header, mut rendered_content) =
        outputs_with(&[raw_content], HashMap::new(), &cfg);
    let expected_output = stylesheet_header + raw_content;
    assert_eq!(expected_output, rendered_content.pop().unwrap());
}

#[test]
fn rendering_delimiter_in_block_code_when_block_delimiter_starts_with_backtick() {
    let raw_content = r#"````
    ```math
    $\omega$
    ```
    ````
    "#;
    let cfg = Config {
        block_delimiter: Delimiter {
            left: "```math".into(),
            right: "```".into(),
        },
        ..Config::default()
    };
    let (stylesheet_header, mut rendered_content) =
        outputs_with(&[raw_content], HashMap::new(), &cfg);
    let expected_output = stylesheet_header + raw_content;
    assert_eq!(expected_output, rendered_content.pop().unwrap());
}

#[test]
fn rendering_delimiter_in_inline_code_when_block_delimiter_starts_with_backtick() {
    let raw_content = r"`$\omega$`";
    let cfg = Config {
        block_delimiter: Delimiter {
            left: "```math".into(),
            right: "```".into(),
        },
        ..Config::default()
    };
    let (stylesheet_header, mut rendered_content) =
        outputs_with(&[raw_content], HashMap::new(), &cfg);
    let expected_output = stylesheet_header + raw_content;
    assert_eq!(expected_output, rendered_content.pop().unwrap());
}

#[test]
fn fenced_code() {
    let raw_content = r"`\` and `` ` `` $\Leftarrow$
```
`\` and `` ` ``
```
while ` ``` ` and ````` ```` ````` $\Leftarrow$
``````
` ``` ` and ````` ```` `````
``````
$$
\Uparrow
$$";
    let cfg = Config::default();
    let (stylesheet_header, rendered_content) =
        outputs_with(&[raw_content], HashMap::new(), &cfg);
    let body = rendered_content[0].strip_prefix(&stylesheet_header).unwrap();
    assert!(body.contains("```"));
    assert!(body.matches("⇐").count() == 2);
    assert!(body.contains("katex-display"));
    assert!(body.contains('⇑'));
}

#[test]
fn inline_rendering_with_custom_delimiter() {
    let raw_content = r"These $\(a\times b\) are from
\[
\int_0^abdx
\]";
    let cfg = Config {
            inline_delimiter: Delimiter {
                left: r"\(".into(),
                right: r"\)".into(),
            },
            block_delimiter: Delimiter {
                left: r"\[".into(),
                right: r"\]".into(),
            },
            ..Config::default()
        };
    let (stylesheet_header, rendered_content) = outputs_with(
        &[raw_content],
        HashMap::new(),
        &cfg,
    );
    let body = rendered_content[0].strip_prefix(&stylesheet_header).unwrap();
    assert!(body.contains("These $"));
    assert!(body.contains('×'));
    assert!(body.contains(" are from"));
    assert!(body.contains("katex-display"));
    assert!(body.contains('∫'));
}
