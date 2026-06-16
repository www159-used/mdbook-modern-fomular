# 用法

在书的 `book.toml` 中启用预处理器：

```toml
[preprocessor.modern-fomular]
after = ["links"]
```

`after = ["links"]` 确保在 mdBook 的 links 预处理器之后运行。

然后照常构建：

```bash
mdbook build
mdbook serve
```

预处理器会在每章内容前注入 KaTeX 样式表（可通过 `no-css = true` 关闭）。

## Markdown 写法

| 类型 | 写法 | 示例 |
|------|------|------|
| 行内 | `$...$` | `$E = mc^2$` |
| 块级 | `$$...$$` | 见下方 |

块级公式：

```markdown
$$
\sum_{i=1}^{n} i = \frac{n(n+1)}{2}
$$
```

## 代码块中的公式

围栏代码块和行内代码里的 `$` **不会**被当作公式分隔符，例如 `` `$x$` `` 会原样保留。

## 日志

默认只输出 `warn` 级别日志到 stderr。可通过环境变量调整：

```bash
RUST_LOG=debug mdbook build
```
