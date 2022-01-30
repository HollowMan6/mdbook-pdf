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

[中文](README_CN.md)

[Blog](https://hollowmansblog.wordpress.com/2022/01/30/mdbook-pdf-a-mdbook-backend-for-generating-pdf-files/)

A backend for [mdBook](https://github.com/rust-lang/mdBook) written in Rust for generating PDF based on [headless chrome](https://github.com/atroche/rust-headless-chrome) and [Chrome DevTools Protocol](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-printToPDF).

## Installation & Usage
Since it's a plugin (backend) for [mdBook](https://github.com/rust-lang/mdBook), first of all you should ensure that `mdbook` is available.

If your machine's architecture is `x86_64`, or you are using Linux for `ARM64`, check the successful [build GitHub Actions workflows](https://github.com/HollowMan6/mdbook-pdf/actions/workflows/build.yml?query=is%3Asuccess), click into the latest one, and then you can get a binary from the Artifacts (including `Windows`, `Linux`, `macOS`).

Otherwise, make sure the [rust compiling environment](https://www.rust-lang.org/tools/install) is available, execute `cargo install mdbook-pdf` to compile and install.

If you want to compile the latest version, make sure the Rust build environment is available (`cargo build`).
Then run `git clone https://github.com/HollowMan6/mdbook-pdf.git`, in the cloned folder, run `cargo build --release` , get the executable in `target/release/`, and put it in PATH.

For running, have Google Chrome / Chromium / Microsoft Edge available (installed at the default location, in PATH or binary location configured) as currently, automatically downloading Chromium binary [isn't available](https://github.com/atroche/rust-headless-chrome/issues/286) (will update once upstream fixes such support).

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

## Configuration
Support customize PDF paper orientation, scale of the webpage rendering, paper width and height, page margins, generated PDF page ranges, whether to display header and footer as well as customize their formats, and more.

Check [book.toml](test_doc/book.toml#L10-L33) and comments for details for the available configurations of `[output.pdf]`.

## Credits
This project relies on [headless_chrome](https://github.com/atroche/rust-headless-chrome). Because the new version has not been released, and the default timeout is not friendly to PDF generation, I use my [Fork version](https://github.com/HollowMan6/rust-headless-chrome) to published [mdbook-pdf-headless_chrome](https://crates.io/crates/mdbook-pdf-headless_chrome) for expanding the relevant timeout to 300 seconds as a submodule of this project, thus enabling the project to be published on [Crates.io](https://crates.io/) as well.
