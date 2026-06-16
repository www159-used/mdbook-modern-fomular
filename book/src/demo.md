# 公式示例

本页由 **mdbook-modern-fomular** 在构建时渲染，用于验证预处理器工作正常。

## 行内公式

欧拉公式：$e^{i\pi} + 1 = 0$。

向量内积：$\langle a, b \rangle = \sum_i a_i b_i$。

## 块级公式

高斯积分：

$$
\int_{-\infty}^{\infty} e^{-x^2} \, dx = \sqrt{\pi}
$$

矩阵：

$$
\begin{pmatrix}
a & b \\
c & d
\end{pmatrix}
\begin{pmatrix}
x \\ y
\end{pmatrix}
=
\begin{pmatrix}
ax + by \\ cx + dy
\end{pmatrix}
$$

## 多行对齐

$$
\begin{aligned}
f(x) &= x^2 \\
f'(x) &= 2x
\end{aligned}
$$
