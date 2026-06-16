mod common;

use std::collections::HashMap;

use mdbook_modern_fomular::Config;

use common::{output_for, outputs_with};

#[test]
fn dollar_escaping_inside_expr() {
    let raw_content = r"We randomly assign: $r \xleftarrow{\$} G $.";
    let (stylesheet_header, rendered_content) = output_for(raw_content);
    let expected_output = stylesheet_header +
    "We randomly assign: <span class=\"katex\"><span class=\"katex-html\" aria-hidden=\"true\"><span class=\"base\"><span class=\"strut\" style=\"height:1.158em;vertical-align:-0.011em;\"></span><span class=\"mord mathnormal\" style=\"margin-right:0.02778em;\">r</span><span class=\"mspace\" style=\"margin-right:0.2778em;\"></span><span class=\"mrel x-arrow\"><span class=\"vlist-t vlist-t2\"><span class=\"vlist-r\"><span class=\"vlist\" style=\"height:1.147em;\"><span style=\"top:-3.322em;\"><span class=\"pstrut\" style=\"height:2.7em;\"></span><span class=\"sizing reset-size6 size3 mtight x-arrow-pad\"><span class=\"mord mtight\"><span class=\"mord mtight\">$</span></span></span></span><span class=\"svg-align\" style=\"top:-2.689em;\"><span class=\"pstrut\" style=\"height:2.7em;\"></span><span class=\"hide-tail\" style=\"height:0.522em;min-width:1.469em;\"><svg xmlns=\"http://www.w3.org/2000/svg\" width='400em' height='0.522em' viewBox='0 0 400000 522' preserveAspectRatio='xMinYMin slice'><path d='M400000 241H110l3-3c68.7-52.7 113.7-120  135-202 4-14.7 6-23 6-25 0-7.3-7-11-21-11-8 0-13.2.8-15.5 2.5-2.3 1.7-4.2 5.8 -5.5 12.5-1.3 4.7-2.7 10.3-4 17-12 48.7-34.8 92-68.5 130S65.3 228.3 18 247 c-10 4-16 7.7-18 11 0 8.7 6 14.3 18 17 47.3 18.7 87.8 47 121.5 85S196 441.3 208  490c.7 2 1.3 5 2 9s1.2 6.7 1.5 8c.3 1.3 1 3.3 2 6s2.2 4.5 3.5 5.5c1.3 1 3.3  1.8 6 2.5s6 1 10 1c14 0 21-3.7 21-11 0-2-2-10.3-6-25-20-79.3-65-146.7-135-202  l-3-3h399890zM100 241v40h399900v-40z'/></svg></span></span></span><span class=\"vlist-s\">\u{200b}</span></span><span class=\"vlist-r\"><span class=\"vlist\" style=\"height:0.011em;\"><span></span></span></span></span></span><span class=\"mspace\" style=\"margin-right:0.2778em;\"></span></span><span class=\"base\"><span class=\"strut\" style=\"height:0.6833em;\"></span><span class=\"mord mathnormal\">G</span></span></span></span>.";
    assert_eq!(expected_output, rendered_content);
}

#[test]
fn inline_rendering() {
    let (stylesheet_header, rendered_content) =
        output_for(r"Some text, $\nabla f(x) \in \mathbb{R}^n$, and more text.");
    let expected_output=stylesheet_header+"Some text, <span class=\"katex\"><span class=\"katex-html\" aria-hidden=\"true\"><span class=\"base\"><span class=\"strut\" style=\"height:1em;vertical-align:-0.25em;\"></span><span class=\"mord\">∇</span><span class=\"mord mathnormal\" style=\"margin-right:0.10764em;\">f</span><span class=\"mopen\">(</span><span class=\"mord mathnormal\">x</span><span class=\"mclose\">)</span><span class=\"mspace\" style=\"margin-right:0.2778em;\"></span><span class=\"mrel\">∈</span><span class=\"mspace\" style=\"margin-right:0.2778em;\"></span></span><span class=\"base\"><span class=\"strut\" style=\"height:0.6889em;\"></span><span class=\"mord\"><span class=\"mord mathbb\">R</span><span class=\"msupsub\"><span class=\"vlist-t\"><span class=\"vlist-r\"><span class=\"vlist\" style=\"height:0.6644em;\"><span style=\"top:-3.063em;margin-right:0.05em;\"><span class=\"pstrut\" style=\"height:2.7em;\"></span><span class=\"sizing reset-size6 size3 mtight\"><span class=\"mord mathnormal mtight\">n</span></span></span></span></span></span></span></span></span></span></span>, and more text.";
    assert_eq!(expected_output, rendered_content);
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
    let expected_output = stylesheet_header + raw_content;
    assert_eq!(expected_output, rendered_content);
}

#[test]
fn invalid_expr_display() {
    let raw_content = r"$$ \< $$";
    let (stylesheet_header, rendered_content) = output_for(raw_content);
    let expected_output = stylesheet_header + raw_content;
    assert_eq!(expected_output, rendered_content);
}

#[test]
fn escaping_backtick() {
    let raw_content = r"\`$\omega$\`";
    let (stylesheet_header, rendered_content) = output_for(raw_content);
    let expected_output = stylesheet_header + "\\`<span class=\"katex\"><span class=\"katex-html\" aria-hidden=\"true\"><span class=\"base\"><span class=\"strut\" style=\"height:0.4306em;\"></span><span class=\"mord mathnormal\" style=\"margin-right:0.03588em;\">ω</span></span></span></span>\\`";
    assert_eq!(expected_output, rendered_content);
}

#[test]
fn include_src() {
    let raw_content = r"Define $f(x)$:

$$
f(x)=x^2\\

x\in\R
$$";
    let (stylesheet_header, rendered_content) = outputs_with(
        &[raw_content],
        HashMap::new(),
        Config {
            include_src: true,
            ..Config::default()
        },
    );
    assert_eq!(stylesheet_header + "Define <data class=\"katex-src\" value=\"f(x)\"><span class=\"katex\"><span class=\"katex-html\" aria-hidden=\"true\"><span class=\"base\"><span class=\"strut\" style=\"height:1em;vertical-align:-0.25em;\"></span><span class=\"mord mathnormal\" style=\"margin-right:0.10764em;\">f</span><span class=\"mopen\">(</span><span class=\"mord mathnormal\">x</span><span class=\"mclose\">)</span></span></span></span></data>:\n\n<data class=\"katex-src\" value=\"&#10;f(x)=x^2\\\\&#10;&#10;x\\in\\R&#10;\"><span class=\"katex-display\"><span class=\"katex\"><span class=\"katex-html\" aria-hidden=\"true\"><span class=\"base\"><span class=\"strut\" style=\"height:1em;vertical-align:-0.25em;\"></span><span class=\"mord mathnormal\" style=\"margin-right:0.10764em;\">f</span><span class=\"mopen\">(</span><span class=\"mord mathnormal\">x</span><span class=\"mclose\">)</span><span class=\"mspace\" style=\"margin-right:0.2778em;\"></span><span class=\"mrel\">=</span><span class=\"mspace\" style=\"margin-right:0.2778em;\"></span></span><span class=\"base\"><span class=\"strut\" style=\"height:0.8641em;\"></span><span class=\"mord\"><span class=\"mord mathnormal\">x</span><span class=\"msupsub\"><span class=\"vlist-t\"><span class=\"vlist-r\"><span class=\"vlist\" style=\"height:0.8641em;\"><span style=\"top:-3.113em;margin-right:0.05em;\"><span class=\"pstrut\" style=\"height:2.7em;\"></span><span class=\"sizing reset-size6 size3 mtight\"><span class=\"mord mtight\">2</span></span></span></span></span></span></span></span></span><span class=\"mspace newline\"></span><span class=\"base\"><span class=\"strut\" style=\"height:0.5782em;vertical-align:-0.0391em;\"></span><span class=\"mord mathnormal\">x</span><span class=\"mspace\" style=\"margin-right:0.2778em;\"></span><span class=\"mrel\">∈</span><span class=\"mspace\" style=\"margin-right:0.2778em;\"></span></span><span class=\"base\"><span class=\"strut\" style=\"height:0.6889em;\"></span><span class=\"mord mathbb\">R</span></span></span></span></span></data>", rendered_content[0]);
}

#[test]
fn renders_vmatrix() {
    let raw = r"$$\begin{vmatrix}a&b\\c&d\end{vmatrix}$$";
    let (stylesheet, rendered) = output_for(raw);
    assert!(rendered.contains("katex-display"));
    assert!(rendered.starts_with(&stylesheet));
}
