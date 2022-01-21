# mdbook-pdf

[![last-commit](https://img.shields.io/github/last-commit/HollowMan6/mdbook-pdf)](https://github.com/HollowMan6/mdbook-pdf/graphs/commit-activity)
![mdbook-pdf build](https://github.com/HollowMan6/mdbook-pdf/workflows/mdbook-pdf%20build/badge.svg)
![mdbook-pdf test](https://github.com/HollowMan6/mdbook-pdf/workflows/mdbook-pdf%20test/badge.svg)

[![Followers](https://img.shields.io/github/followers/HollowMan6?style=social)](https://github.com/HollowMan6?tab=followers)
[![watchers](https://img.shields.io/github/watchers/HollowMan6/mdbook-pdf?style=social)](https://github.com/HollowMan6/mdbook-pdf/watchers)
[![stars](https://img.shields.io/github/stars/HollowMan6/mdbook-pdf?style=social)](https://github.com/HollowMan6/mdbook-pdf/stargazers)
[![forks](https://img.shields.io/github/forks/HollowMan6/mdbook-pdf?style=social)](https://github.com/HollowMan6/mdbook-pdf/network/members)

[![Open Source Love](https://img.shields.io/badge/-%E2%9D%A4%20Open%20Source-Green?style=flat-square&logo=Github&logoColor=white&link=https://hollowman6.github.io/fund.html)](https://hollowman6.github.io/fund.html)
[![GPL Licence](https://img.shields.io/badge/license-GPL-blue)](https://opensource.org/licenses/GPL-3.0/)
[![Repo-Size](https://img.shields.io/github/repo-size/HollowMan6/mdbook-pdf.svg)](https://github.com/HollowMan6/mdbook-pdf/archive/master.zip)

[English](README.md)

用 Rust 编写的 [mdBook](https://github.com/rust-lang/mdBook) 后端，基于[headless chrome](https://github.com/atroche/rust-headless-chrome)和[Chrome开发工具协议](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-printToPDF)生成PDF。

## 用法
如果您的计算机的架构为`x86_64`，请检查成功的[build GitHub Actions workflows](https://github.com/HollowMan6/mdbook-pdf/actions/workflows/build.yml?query=is%3Asuccess)，单击最新的一次运行记录，然后您可以从 Artifacts中获取二进制文件（包括 `Windows`、`Linux`、`macOS`）。

否则，请确保 Rust 编译环境可用（`cargo build`），然后在此文件夹中运行`cargo build --release`，在`target/release/`中获取可执行文件，并将其放入PATH。

为了使得程序能够正常运行，请确保计算机上安装了 Google Chrome / Chromium / Microsoft Edge，（安装在默认的位置，在当前的PATH中，或配置了二进制文件位置），因为现在自动下载 Chromium 功能还[不可用](https://github.com/atroche/rust-headless-chrome/issues/286).

请确保您的`book.toml`中存在以下内容:

```toml
[output.html]

[output.pdf]
```

而且，`[output.html.print]`也没有被禁用（默认情况下应该是启用的，所以如果您的`book.toml`中没有出现以下行，请不要担心）。

```toml
[output.html.print]
enable = true
```

最后，您可以使用 `mdbook build` 命令生成书籍并获取PDF文件，您的PDF文件将被存放在`book/pdf/output.put`。

## 配置
支持自定义PDF纸张方向、页面缩放比例、纸张宽度和高度、页面边距、生成的PDF页面范围、是否显示页眉和页脚以及自定义其格式等。

查看 [book.toml](test_doc/book.toml#L10-L33) 以了解 `[output.pdf]` 可用配置的详细信息。

## Crates.io
由于本软件的依赖使用了来自 [headless chrome GitHub fork](https://github.com/HollowMan6/rust-headless-chrome) 的补丁，因而不会在 [crates.io](https://crates.io) 中发布。
