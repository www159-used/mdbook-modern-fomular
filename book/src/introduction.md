# 简介

**mdbook-modern-fomular** 是一个 [mdBook](https://rust-lang.github.io/mdBook/) 预处理器，在 `mdbook build` 阶段把 Markdown 里的 LaTeX 数学公式用 [KaTeX](https://katex.org/) 渲染成 HTML。

与在浏览器里用 JavaScript 渲染不同，本工具在构建时完成渲染，生成的静态站点无需额外脚本即可显示公式。

## 特点

- 支持行内公式 `$...$` 与块级公式 `$$...$$`
- 可自定义分隔符、宏定义文件
- 兼容 mdBook **0.5.x**
- 渲染失败时保留原文，不中断构建

## 示例

行内：设 $f(x) = x^2$，则 $\nabla f(x) = 2x$。

块级：

$$
\int_0^1 x^2 \, dx = \frac{1}{3}
$$
