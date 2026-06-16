# 配置

所有选项写在 `book.toml` 的 `[preprocessor.modern-fomular]` 段中。

## 常用选项

```toml
[preprocessor.modern-fomular]
after = ["links"]

# 自定义分隔符
inline-delimiter = { left = "$", right = "$" }
block-delimiter = { left = "$$", right = "$$" }

# 宏定义文件（相对书根目录）
macros = "macros.txt"

# 在 HTML 中保留原始 LaTeX 源码
include-src = false

# 不注入 KaTeX CSS（需自行引入样式）
no-css = false
```

## 宏文件格式

每行一条，格式为 `宏名:展开式`：

```text
\grad:\nabla
\R:\mathbb{R}^#1
```

## KaTeX 选项

| 选项 | 默认值 | 说明 |
|------|--------|------|
| `output` | `"html"` | `html` / `mathml` / `htmlAndMathml` |
| `throw-on-error` | `true` | 无效 LaTeX 是否报错 |
| `error-color` | `"#cc0000"` | 错误颜色 |
| `leqno` | `false` | 公式编号靠左 |
| `fleqn` | `false` | 块级公式左对齐 |
| `trust` | `false` | 是否信任用户输入 |
| `max-expand` | `1000` | 宏展开次数上限 |

完整列表见源码中的 `Config` 结构体。
