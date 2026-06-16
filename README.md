# mdbook-modern-fomular

mdBook 预处理器：在构建时用 [KaTeX](https://katex.org/) 将 LaTeX 数学公式预渲染为 HTML。

**文档：** https://www159-used.github.io/mdbook-modern-fomular/

## 安装

```bash
cargo install mdbook-modern-fomular
```

需要 mdBook **0.5.x**。

## 配置

在 `book.toml` 中加入：

```toml
[preprocessor.modern-fomular]
after = ["links"]
```

## 许可

AGPL-3.0-or-later
