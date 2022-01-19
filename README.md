# mdbook-pdf

[![last-commit](https://img.shields.io/github/last-commit/HollowMan6/mdbook-pdf)](https://github.com/HollowMan6/mdbook-pdf/graphs/commit-activity)
![mdbook-pdf test](https://github.com/HollowMan6/mdbook-pdf/workflows/mdbook-pdf%20test/badge.svg)

[![Followers](https://img.shields.io/github/followers/HollowMan6?style=social)](https://github.com/HollowMan6?tab=followers)
[![watchers](https://img.shields.io/github/watchers/HollowMan6/mdbook-pdf?style=social)](https://github.com/HollowMan6/mdbook-pdf/watchers)
[![stars](https://img.shields.io/github/stars/HollowMan6/mdbook-pdf?style=social)](https://github.com/HollowMan6/mdbook-pdf/stargazers)
[![forks](https://img.shields.io/github/forks/HollowMan6/mdbook-pdf?style=social)](https://github.com/HollowMan6/mdbook-pdf/network/members)

[![Open Source Love](https://img.shields.io/badge/-%E2%9D%A4%20Open%20Source-Green?style=flat-square&logo=Github&logoColor=white&link=https://hollowman6.github.io/fund.html)](https://hollowman6.github.io/fund.html)
[![GPL Licence](https://img.shields.io/badge/license-GPL-blue)](https://opensource.org/licenses/GPL-3.0/)
[![Repo-Size](https://img.shields.io/github/repo-size/HollowMan6/mdbook-pdf.svg)](https://github.com/HollowMan6/mdbook-pdf/archive/master.zip)

[![Total alerts](https://img.shields.io/lgtm/alerts/g/HollowMan6/mdbook-pdf.svg?logo=lgtm&logoWidth=18)](https://lgtm.com/projects/g/HollowMan6/mdbook-pdf/alerts/)
[![Language grade: Python](https://img.shields.io/lgtm/grade/python/g/HollowMan6/mdbook-pdf.svg?logo=lgtm&logoWidth=18)](https://lgtm.com/projects/g/HollowMan6/mdbook-pdf/context:python)

A backend for mdbook written in Python for generating PDF based on [Chrome DevTools Protocol](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-printToPDF).

[Python library dependency](https://github.com/HollowMan6/mdbook-pdf/network/dependencies)

## Usage

Put [mdbook-pdf](mdbook-pdf) in PATH.
Have google-chrome/chromium available (in PATH or location configured).

Ensure you have installed [python selenium library](https://github.com/HollowMan6/mdbook-pdf/network/dependencies),
corresponding chromedriver is in PATH or in the book repo or location configured.

Build it with `mdbook build` command.
Make sure the following exists in your `book.toml`:

```toml
[output.html]

[output.html.print]
enable = true

[output.pdf]
```

if you are using Windows, put this script in the book repo,
add the following line to `[output.pdf]` in your `book.toml`:

```toml
command = "python ../../mdbook-pdf"
```

## Configuration

Check [book.toml](book.toml#L13) for available configurations of `[output.pdf]`.

## Known issue

Sometimes the program may crash with errors like this:

```bash
selenium.common.exceptions.WebDriverException: Message: unknown error: session deleted because of page crash
from unknown error: cannot determine loading status
from tab crashed
```

This may be led by resources shortage, you can retry building it and there are chances for it to work.

## To-do

Rewrite the whole thing in Rust, directly call to Chrome DevTools Protocol instead of using selenium.
