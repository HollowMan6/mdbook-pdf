# mdbook-pdf

[![last-commit](https://img.shields.io/github/last-commit/HollowMan6/mdbook-pdf)](https://github.com/HollowMan6/mdbook-pdf/graphs/commit-activity)
[![release-date](https://img.shields.io/github/release-date/HollowMan6/mdbook-pdf)](https://github.com/HollowMan6/mdbook-pdf/releases)
[![Crate](https://img.shields.io/crates/v/mdbook-pdf.svg)](https://crates.io/crates/mdbook-pdf)
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

[v0.1.0 开发文档](https://openeuler.feishu.cn/docs/doccnQFaB5OLABMDjhWSE1siX5b#)

用 Rust 编写的 [mdBook](https://github.com/rust-lang/mdBook) 后端，基于[headless chrome](https://github.com/atroche/rust-headless-chrome)和[Chrome开发工具协议](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-printToPDF)生成PDF。

## 用法
如果您的计算机的架构为`x86_64`，或者`Arm64`版本的Linux，请检查成功的[build GitHub Actions workflows](https://github.com/HollowMan6/mdbook-pdf/actions/workflows/build.yml?query=is%3Asuccess)，单击最新的一次运行记录，然后您可以从 Artifacts中获取二进制文件（包括 `Windows`、`Linux`、`macOS`）。

否则，执行`cargo install mdbook-pdf`编译安装即可。

如果需要最新版的编译二进制文件，请确保 Rust 编译环境可用（`cargo build`），然后执行`git clone --recursive https://github.com/HollowMan6/mdbook-pdf.git`，在然后在克隆下来的文件夹中运行`cargo build --release`，在`target/release/`中获取可执行文件，并将其放入PATH。

为了使得程序能够正常运行，请确保计算机上安装了 Google Chrome / Chromium / Microsoft Edge，（安装在默认的位置，在当前的PATH中，或配置了二进制文件位置），因为现在自动下载 Chromium 功能还[不可用](https://github.com/atroche/rust-headless-chrome/issues/286).

- 在Windows 10及以上该程序无需安装任何额外软件即可正常生成 PDF，因为 Microsoft Edge 是 Windows 系统自带的浏览器。当然如果考虑到对没有自带安装 Edge 的老版本Windows的支持，在电脑上安装一个 Google Chrome 即可。
- 在 macOS 中需要下载并安装 [Google Chrome](https://www.google.com/chrome/) 或者 [Microsoft Edge](https://www.microsoft.com/zh-cn/edge) 或者 Chromium。
- 在 Linux 中安装Google Chrome / Chromium / Microsoft Edge 浏览器中的任意一个即可，推荐安装 Chromium，该软件包在您的发行版中一般名称为 `chromium` 或 `chromium-browser`（注意，在 Ubuntu 18.04 之后需要通过 `snap` 安装 `chromium-browser`）。

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

一个最简单的`book.toml`文件示例如下：

```toml
[book]
title = "An Example"

[output.html]

[output.pdf]
```

最后，您可以使用 `mdbook build` 命令生成书籍并获取PDF文件，您的PDF文件将被存放在`book/pdf/output.put`。

(注：因为注意到书名中很容易出现特殊字符，比如说`:`，这样的话会导致生成相关PDF文件失败，所以文件名没有使用`<书名>.pdf`这种的形式，而是统一用`output.pdf`)

## 配置
支持自定义PDF纸张方向、页面缩放比例、纸张宽度和高度、页面边距、生成的PDF页面范围、是否显示页眉和页脚以及自定义其格式等。

查看 [book.toml](test_doc/book.toml#L10-L33) 以了解 `[output.pdf]` 可用配置的详细信息。

### 具体参数详解
- trying-times

接受输入一个整型数，默认为`1`。其指定假如发生PDF生成失败的情况重试的次数。

- browser-binary-path

接受输入一个字符串，默认为空`''`，程序自动判断路径。其指定浏览器可执行文件路径。

本程序支持最新的基于Chromium的浏览器，不支持Safari和Firefox。如果你需要指定，请指定完整的路径，比如说`/usr/bin/foo`。如果指定了错误的可执行文件，则很可能会出现超时错误或者直接报错。

- landscape

接受输入一个布尔值，默认为`false`。其指定PDF纸张方向，`true`为横向，`false`为纵向。

- display-header-footer

接受输入一个布尔值，默认为`false`。其指定是否显示页眉和页脚，`true`为显示，`false`为不显示。

- print-background
  
接受输入一个布尔值，默认为`false`。其指定是否在PDF中显示背景图片，`true`为显示，`false`为不显示。

- scale

接受输入一个数字，默认为`1`。其指定缩放因子，例如指定值为`1.25`，则将页面缩放125%。

- paper-width

接受输入一个数字，默认为`8.5`。其指定页面宽度的英尺数，如果需要使用A4纸请将此值设为`8`。

- paper-height

接受输入一个数字，默认为`11`。其指定页面高度的英尺数，如果需要使用A4纸请将此值设为`10`。

- margin-top

接受输入一个数字，默认为`1`。其指定页面上边距的厘米数。

- margin-bottom

接受输入一个数字，默认为`1`。其指定页面下边距的厘米数。

- margin-left

接受输入一个数字，默认为`1`。其指定页面左边距的厘米数。

- margin-right

接受输入一个数字，默认为`1`。其指定页面右边距的厘米数。

- page-range

接受输入一个字符串，默认为空`''`，即不截取PDF页面。其指定生成PDF文件页面截取范围，支持指定常见打印机格式页面范围，如`'1-5, 8, 11-13'`则是将第1到5页以及第8页和第11到13页截取出来生成。

- ignore-invalid-page-ranges

接受输入一个布尔值，默认为`false`。其指定，如果上面指定的PDF文件页面截取范围格式正确，但是实际无法按照语义执行，是否忽略。`true`为忽略，生成全部PDF页面，`false`为进行报错，如在指定`3-2`这种情况下将会报错，PDF生成失败。

- header-template

接受输入一个字符串。其指定PDF文件页眉的HTML模板。其值应该是一个有效的HTML标记，并使用以下类从而将对应值插入其中：

   - date: 格式化后的PDF生成日期
   - title: 书的标题
   - url: PDF文件存放路径
   - pageNumber: 当前页号
   - totalPages: 总共页数

例如，`'<span class=title></span>'` 将生成一个包含标题的页眉。

- footer-template

接受输入一个字符串。其指定PDF文件页脚的HTML模板。其值的格式同header-template。

- prefer-css-page-size

接受输入一个布尔值，默认为`false`。其指定是否使用 CSS 定义的页面大小。`true`为使用，`false`时页面将通过缩放来适应纸张大小。

## 致谢

本项目依赖于 [headless_chrome](https://github.com/atroche/rust-headless-chrome)。因其一直未发布新版，且默认超时时间对PDF生成不友好，所以使用了我的[Fork 版本](https://github.com/HollowMan6/rust-headless-chrome)发布了[mdbook-pdf-headless_chrome](https://crates.io/crates/mdbook-pdf-headless_chrome)，将相关超时时间扩大为300秒，作为本项目的子模块，从而使得项目能够发布在[Crates.io](https://crates.io/)
