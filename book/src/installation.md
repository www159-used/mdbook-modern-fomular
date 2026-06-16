# 安装

## 依赖

- [Rust](https://www.rust-lang.org/) 工具链
- [mdBook](https://github.com/rust-lang/mdBook) **0.5.x**

```bash
cargo install mdbook --locked
```

## 安装预处理器

```bash
cargo install mdbook-modern-fomular --locked
```

安装后应能在 PATH 中找到 `mdbook-modern-fomular`。

## 从源码构建

```bash
git clone https://github.com/www159-used/mdbook-modern-fomular.git
cd mdbook-modern-fomular
cargo install --path . --locked
```
