# mdbook-pdf
[![](https://dockeri.co/image/hollowman6/mdbook-pdf)](https://hub.docker.com/r/hollowman6/mdbook-pdf)

[![last-commit](https://img.shields.io/github/last-commit/HollowMan6/mdbook-pdf)](https://github.com/HollowMan6/mdbook-pdf/graphs/commit-activity)
[![release-date](https://img.shields.io/github/release-date/HollowMan6/mdbook-pdf)](https://github.com/HollowMan6/mdbook-pdf/releases)
[![Crate](https://img.shields.io/crates/v/mdbook-pdf.svg)](https://crates.io/crates/mdbook-pdf)
![mdbook-pdf build](https://github.com/HollowMan6/mdbook-pdf/workflows/mdbook-pdf%20build/badge.svg)
![mdbook-pdf test](https://github.com/HollowMan6/mdbook-pdf/workflows/mdbook-pdf%20test/badge.svg)
![Python package](https://github.com/HollowMan6/mdbook-pdf/workflows/Python%20package/badge.svg)

[![Followers](https://img.shields.io/github/followers/HollowMan6?style=social)](https://github.com/HollowMan6?tab=followers)
[![watchers](https://img.shields.io/github/watchers/HollowMan6/mdbook-pdf?style=social)](https://github.com/HollowMan6/mdbook-pdf/watchers)
[![stars](https://img.shields.io/github/stars/HollowMan6/mdbook-pdf?style=social)](https://github.com/HollowMan6/mdbook-pdf/stargazers)
[![forks](https://img.shields.io/github/forks/HollowMan6/mdbook-pdf?style=social)](https://github.com/HollowMan6/mdbook-pdf/network/members)

[![Open Source Love](https://img.shields.io/badge/-%E2%9D%A4%20Open%20Source-Green?style=flat-square&logo=Github&logoColor=white&link=https://hollowman6.github.io/fund.html)](https://hollowman6.github.io/fund.html)
[![GPL Licence](https://img.shields.io/badge/license-GPL-blue)](https://opensource.org/licenses/GPL-3.0/)
[![Repo-Size](https://img.shields.io/github/repo-size/HollowMan6/mdbook-pdf.svg)](https://github.com/HollowMan6/mdbook-pdf/archive/master.zip)

[中文](https://github.com/HollowMan6/mdbook-pdf/blob/main/README_CN.md)

[Blog](https://hollowmansblog.wordpress.com/2022/01/30/mdbook-pdf-a-mdbook-backend-for-generating-pdf-files/)

A backend for [mdBook](https://github.com/rust-lang/mdBook) written in Rust for generating PDF based on [headless chrome](https://github.com/atroche/rust-headless-chrome) and [Chrome DevTools Protocol](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-printToPDF).

## Installation & Usage
Since it's a plugin (backend) for [mdBook](https://github.com/rust-lang/mdBook), first of all you should ensure that `mdbook` is available.

If your machine's architecture is `x86_64`, or you are using Linux for `ARM64`, check the successful [build GitHub Actions workflows](https://github.com/HollowMan6/mdbook-pdf/actions/workflows/build.yml?query=is%3Asuccess)/[release](https://github.com/HollowMan6/mdbook-pdf/releases), click into the latest one, and then you can get a binary from the Artifacts (including `Windows`, `Linux`, `macOS`).

Otherwise, make sure the [rust compiling environment](https://www.rust-lang.org/tools/install) is available, execute `cargo install mdbook-pdf` to compile and install.

If you want to compile the latest version, make sure the Rust build environment is available (`cargo build`).
Then run `git clone https://github.com/HollowMan6/mdbook-pdf.git`, in the cloned folder, run `cargo build --release` , get the executable in `target/release/`, and put it in PATH.

For running, please have Google Chrome / Chromium / Microsoft Edge already available (installed at the default location, in PATH or binary location configured). If not, and `mdbook-pdf` has the `fetch` feature enabled (It is not enabled by default, you need to use `cargo install mdbook-pdf --features fetch` to recompile for enabling), the program will try to automatically download the Chromium browser and run it (Note: if you are on Linux, there may be problems if chromium dependencies are not satisfied / using non-x86_64 architectures).

- On Windows 10 and above, the program can generate PDF normally without installing any additional software, because Microsoft Edge is the browser provided with Windows system. Of course, considering the support for the older versions of Windows without Edge, you can install Google Chrome on your computer.
- In MacOS, you need to install [Google Chrome](https://www.google.com/chrome/) / [Microsoft Edge](https://www.microsoft.com/en-us/edge) or Chromium.
- In Linux, you can choose to install any of the Google Chrome / Chromium / Microsoft Edge browsers. It is recommended to install Chromium. The name of this software package in your Linux distribution is commonly `chromium` or `chromium-browser` (Note: for Ubuntu later than 18.04, you have to install `chromium-browser` through `snap`).

Make sure the following exists in your `book.toml`:

```toml
[output.html]

[output.pdf]
```

And also `[output.html.print]` is not disabled (it should be enabled by default, so don't worry if the following lines doesn't occur in you `book.toml`).

```toml
[output.html.print]
enable = true
```

A simplest `book.toml` is as follows:

```toml
[book]
title = "An Example"

[output.html]

[output.pdf]
```

Finally you can build your book and get the PDF file with `mdbook build` command, your PDF file will be available at `book/pdf/output.pdf`.

## Run with Docker
You can also use this [docker image](https://hub.docker.com/r/hollowman6/mdbook-pdf) if your book doesn't have dependencies other than mdBook.

```bash
docker run --rm -v /path/to/book:/book hollowman6/mdbook-pdf
```

## Configuration
Support customize PDF paper orientation, scale of the webpage rendering, paper width and height, page margins, generated PDF page ranges, whether to display header and footer as well as customize their formats, and more.

Check [book.toml](https://github.com/HollowMan6/mdbook-pdf/blob/main/test_doc/book.toml#L10-L36) and comments for details for the available configurations of `[output.pdf]`.

## Common Issues
1. Support for Firefox in `mdbook-pdf`!

Currently, according to [Puppeteer's documentation](https://pptr.dev/#?product=Puppeteer&show=api-pagepdfoptions), [Chrome DevTools Protocol Page.printToPDF](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-printToPDF) is only supported in Chrome headless. An issue has already been filed for this [here](https://github.com/puppeteer/puppeteer/issues/7628).

2. Broken links!

I've already submitted [a PR for mdBook](https://github.com/rust-lang/mdBook/pull/1738) to fix this by making print page (print.html) links link to anchors on the print page, but it's not merged yet. You can try [my PR fork](https://github.com/HollowMan6/mdBook) for this to work.

If you have relative links that link outside the book, please provide the [static hosting site URL](https://github.com/HollowMan6/mdbook-pdf/blob/main/test_doc/book.toml#L17-L18) for it to get fixed.

3. Can you add the bookmark to the PDF reflecting the Table of Contents, just like what [wkhtmltopdf](https://wkhtmltopdf.org/) supported?

This should be realized by Chromium, and an issue has already been filed for this [here](https://bugs.chromium.org/p/chromium/issues/detail?id=781797).

Initial support for the bookmark/outline of the PDF file has already been available ([mdbook-pdf-outline](https://pypi.org/project/mdbook-pdf-outline/)). It is written in Python and is another backend for `mdbook` and should be used with `mdbook-pdf` and ***the [mdbook version](https://github.com/rust-lang/mdBook/pull/1738) mentioned in Common Issues 2 for fixing the broken links in `print.html`***.

You can install this backend by `pip install mdbook-pdf-outline`.

Remember to put the following to ***the end of*** your `book.toml`, ***after [output.pdf]***:

```toml
[output.pdf-outline]
```

If you want to use the table of content just like the one shown in the `print.html` page for PDF file, you can leave the `book.toml` as it is.

If you prefer to use the table of content just like the one generated by `wkhtmltopdf` (generate entries based on the headings), you can turn on the `like-wkhtmltopdf` option by using the following to your `book.toml`:

```toml
[output.pdf-outline]
like-wkhtmltopdf = true
```

Finally, you can find the outlined version at `book/pdf-outline/output.pdf`.

4. Failed to render my book for PDF in `mdbook-pdf`!

Will appreciate if you can report it to the [issue tracker](https://github.com/HollowMan6/mdbook-pdf/issues/new) providing all the traces for `mdbook-pdf` rendering as well as your `book.toml`. You can also provide the link to your book's repository if it's open source.

```bash
RUST_BACKTRACE=full RUST_LOG=trace mdbook build
```
## Credits
- This project relies on [headless_chrome](https://github.com/atroche/rust-headless-chrome). Because the default timeout is not friendly to PDF generation, I use my [forked version](https://github.com/HollowMan6/rust-headless-chrome) to published [mdbook-pdf-headless_chrome](https://crates.io/crates/mdbook-pdf-headless_chrome) for expanding the relevant timeout to 600 seconds, thus enabling the project to be published on [Crates.io](https://crates.io/) as well.
