# cargo-watch

使用 [cargo-watch](https://crates.io/crates/cargo-watch) 工具监听项目文件变化，并自动编译运行，实现开发热重载。

## 安装

实践下来，在不同的操作系统下，安装 `cargo-watch` 有不同的方式。

- MacOS

```bash
# 首先得先安装 binstall
brew install binstall

# 然后安装 cargo-watch
cargo binstall cargo-watch
```

- Linux

```bash
cargo install cargo-watch
```

## 使用

```bash
# 监听项目文件变化，并自动编译运行
cargo watch -x run
```
