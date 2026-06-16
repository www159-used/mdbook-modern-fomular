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
        outputs_with(&[raw_content], HashMap::new(), cfg);
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
        outputs_with(&[raw_content], HashMap::new(), cfg);
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
        outputs_with(&[raw_content], HashMap::new(), cfg);
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
    let (stylesheet_header, rendered_content) =
        outputs_with(&[raw_content], HashMap::new(), Config::default());
    assert_eq!(
        stylesheet_header +
        "`\\` and `` ` `` <span class=\"katex\"><span class=\"katex-html\" aria-hidden=\"true\"><span class=\"base\"><span class=\"strut\" style=\"height:0.3669em;\"></span><span class=\"mrel\">⇐</span></span></span></span>\n```\n`\\` and `` ` ``\n```\nwhile ` ``` ` and ````` ```` ````` <span class=\"katex\"><span class=\"katex-html\" aria-hidden=\"true\"><span class=\"base\"><span class=\"strut\" style=\"height:0.3669em;\"></span><span class=\"mrel\">⇐</span></span></span></span>\n``````\n` ``` ` and ````` ```` `````\n``````\n<span class=\"katex-display\"><span class=\"katex\"><span class=\"katex-html\" aria-hidden=\"true\"><span class=\"base\"><span class=\"strut\" style=\"height:0.8889em;vertical-align:-0.1944em;\"></span><span class=\"mrel\">⇑</span></span></span></span></span>",
        rendered_content[0]
    );
}

#[test]
fn inline_rendering_with_custom_delimiter() {
    let raw_content = r"These $\(a\times b\) are from
\[
\int_0^abdx
\]";
    let (stylesheet_header, rendered_content) = outputs_with(
        &[raw_content],
        HashMap::new(),
        Config {
            inline_delimiter: Delimiter {
                left: r"\(".into(),
                right: r"\)".into(),
            },
            block_delimiter: Delimiter {
                left: r"\[".into(),
                right: r"\]".into(),
            },
            ..Config::default()
        },
    );
    let expected_output = stylesheet_header + "These $<span class=\"katex\"><span class=\"katex-html\" aria-hidden=\"true\"><span class=\"base\"><span class=\"strut\" style=\"height:0.6667em;vertical-align:-0.0833em;\"></span><span class=\"mord mathnormal\">a</span><span class=\"mspace\" style=\"margin-right:0.2222em;\"></span><span class=\"mbin\">×</span><span class=\"mspace\" style=\"margin-right:0.2222em;\"></span></span><span class=\"base\"><span class=\"strut\" style=\"height:0.6944em;\"></span><span class=\"mord mathnormal\">b</span></span></span></span> are from\n<span class=\"katex-display\"><span class=\"katex\"><span class=\"katex-html\" aria-hidden=\"true\"><span class=\"base\"><span class=\"strut\" style=\"height:2.3262em;vertical-align:-0.9119em;\"></span><span class=\"mop\"><span class=\"mop op-symbol large-op\" style=\"margin-right:0.44445em;position:relative;top:-0.0011em;\">∫</span><span class=\"msupsub\"><span class=\"vlist-t vlist-t2\"><span class=\"vlist-r\"><span class=\"vlist\" style=\"height:1.4143em;\"><span style=\"top:-1.7881em;margin-left:-0.4445em;margin-right:0.05em;\"><span class=\"pstrut\" style=\"height:2.7em;\"></span><span class=\"sizing reset-size6 size3 mtight\"><span class=\"mord mtight\">0</span></span></span><span style=\"top:-3.8129em;margin-right:0.05em;\"><span class=\"pstrut\" style=\"height:2.7em;\"></span><span class=\"sizing reset-size6 size3 mtight\"><span class=\"mord mathnormal mtight\">a</span></span></span></span><span class=\"vlist-s\">\u{200b}</span></span><span class=\"vlist-r\"><span class=\"vlist\" style=\"height:0.9119em;\"><span></span></span></span></span></span></span><span class=\"mspace\" style=\"margin-right:0.1667em;\"></span><span class=\"mord mathnormal\">b</span><span class=\"mord mathnormal\">d</span><span class=\"mord mathnormal\">x</span></span></span></span></span>";
    assert_eq!(expected_output, rendered_content[0]);
}
