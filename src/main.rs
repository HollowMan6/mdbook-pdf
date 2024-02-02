/**
 * mdbook-pdf
 * A PDF generator for mdBook using headless Chrome.
 *
 * Author:  Hollow Man <hollowman@opensuse.org>
 * License: GPL-3.0
 *
 * Copyright (C) 2022-2023 Hollow Man (@HollowMan6)
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
use headless_chrome::{types::PrintToPdfOptions, Browser, LaunchOptionsBuilder};
use lazy_static::lazy_static;
use mdbook::renderer::RenderContext;
use regex::Regex;
use std::io::{BufReader, BufWriter, Read, Write};
use std::{ffi::OsStr, fs, io, path::PathBuf, thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    lazy_static! {
        static ref SCHEME_LINK: Regex = Regex::new(r"^[a-z][a-z0-9+.-]*:").unwrap();
        static ref A_LINK: Regex = Regex::new(r#"(<a [^>]*?href=")([^"]+?)""#).unwrap();
    }

    // Receives the data passed to the program via mdbook
    let mut stdin = io::stdin();

    // Get the configs
    let ctx = RenderContext::from_json(&mut stdin).unwrap();
    let cfg: PrintOptions = ctx
        .config
        .get_deserialized_opt("output.pdf")
        .unwrap()
        .unwrap();

    let print_html_path = ctx
        .destination
        .parent()
        .unwrap()
        .join("html")
        .join("print.html")
        .to_owned()
        .to_str()
        .unwrap()
        .to_owned();

    if !PathBuf::from(&print_html_path).exists() {
        println!(
            "PDF generation failed. The print.html file does not exist at {}.",
            print_html_path
        );
        println!("Verify output.html is active and output.html.print.enabled is set to true in your book.toml.");
        return Err(Box::<io::Error>::from(io::Error::new(
            io::ErrorKind::NotFound,
            format!("File not found: {}", print_html_path),
        )));
    }

    // Modify the print.html for custom JS scripts as well as links outside the book.
    let file = fs::OpenOptions::new()
        .read(true)
        .open(print_html_path.clone())
        .unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    // Insert a link to the page div in the print.html to make sure that generated pdf
    // contains the destination for ToC to locate the specific page in pdf.
    let mut toc_fix = "<div style=\"display: none\">".to_owned();
    for item in ctx.book.iter() {
        if let mdbook::book::BookItem::Chapter(chapter) = item {
            let path = chapter.path.clone();
            if let Some(path) = path {
                let print_page_id = {
                    let mut base = path.display().to_string();
                    if base.ends_with(".md") {
                        base.truncate(base.len() - 3);
                    }
                    &base.replace(['/', '\\'], "-").to_ascii_lowercase()
                };
                toc_fix.push_str(&(format!(r##"<a href="#{print_page_id}">{print_page_id}</a>"##)));
            }
        }
    }
    toc_fix.push_str("</div>");

    let script = "</script>

        <!-- Custom JS scripts for mdbook-pdf PDF generation -->
        <script type='text/javascript'>
            let markAllContentHasLoadedForPrinting = () =>
                window.setTimeout(
                    () => {
                        let p = document.createElement('div');
                        p.setAttribute('id', 'content-has-all-loaded-for-mdbook-pdf-generation');
                        document.body.appendChild(p);
                    }, 100
                );

            window.addEventListener('load', () => {
                // Expand all the <details> elements for printing.
                r = document.getElementsByTagName('details');
                for (let i of r)
                    i.open = true;

                try {
                    MathJax.Hub.Register.StartupHook('End', markAllContentHasLoadedForPrinting);
                } catch (e) {
                    markAllContentHasLoadedForPrinting();
                }
            });
        </script>
    "
    .to_owned();

    contents = contents.replacen("</script>", &(script + &toc_fix), 1);
    if !cfg.static_site_url.is_empty() {
        contents = A_LINK
            .replace_all(&contents, |caps: &regex::Captures<'_>| {
                // Ensure that there is no '\' in the link to ensure the following judgement work
                let link = caps[2].replace('\\', "/");
                // Don't modify links with schemes like `https`, and no need to modify pages inside the book.
                if !link.starts_with('#')
                    && !SCHEME_LINK.is_match(&link)
                    && (link.starts_with("../") || link.contains("/../"))
                {
                    let mut fixed_link = String::new();

                    fixed_link.push_str(&cfg.static_site_url);
                    if !fixed_link.ends_with('/') {
                        fixed_link.push('/');
                    }
                    fixed_link.push_str(&link);

                    return format!("{}{}\"", &caps[1], &fixed_link);
                }
                // Otherwise, leave it as-is.
                format!("{}{}\"", &caps[1], &caps[2])
            })
            .into_owned();
    }

    let file = fs::OpenOptions::new()
        .write(true)
        .open(print_html_path.clone())
        .unwrap();
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(contents.as_bytes())?;

    // Used to relieve errors related to timeouts, but now for
    // the patches in my fork, it's not likely to be needed
    let trying_times = cfg.trying_times + 1;

    println!("Generating PDF, please be patient...");

    for time in 1..trying_times {
        let url = format!("file://{}", print_html_path);

        let cloned_cfg = cfg.clone();
        let browser_binary = if cloned_cfg.browser_binary_path.is_empty() {
            None
        } else {
            Some(PathBuf::from(cloned_cfg.browser_binary_path))
        };

        let launch_opts = LaunchOptionsBuilder::default()
            .headless(true)
            .sandbox(false)
            .idle_browser_timeout(Duration::from_secs(cfg.timeout))
            .path(browser_binary)
            .args(vec![
                OsStr::new("--disable-pdf-tagging"),
                OsStr::new("--unlimited-storage"),
                OsStr::new("--webkit-print-color-adjust"),
            ])
            .build()?;

        let pdf_opts = PrintToPdfOptions {
            landscape: Some(cloned_cfg.landscape),
            display_header_footer: Some(cloned_cfg.display_header_footer),
            print_background: Some(cloned_cfg.print_background),
            scale: Some(cloned_cfg.scale),
            paper_width: Some(cloned_cfg.paper_width),
            paper_height: Some(cloned_cfg.paper_height),
            margin_top: Some(cloned_cfg.margin_top),
            margin_bottom: Some(cloned_cfg.margin_bottom),
            margin_left: Some(cloned_cfg.margin_left),
            margin_right: Some(cloned_cfg.margin_right),
            page_ranges: Some(cloned_cfg.page_ranges),
            ignore_invalid_page_ranges: Some(cloned_cfg.ignore_invalid_page_ranges),
            header_template: Some(cloned_cfg.header_template),
            footer_template: Some(cloned_cfg.footer_template),
            prefer_css_page_size: Some(cloned_cfg.prefer_css_page_size),
            transfer_mode: None,
        };

        // Create a new browser window.
        let browser = Browser::new(launch_opts)?;
        let tab = browser.new_tab()?;
        tab.set_default_timeout(std::time::Duration::from_secs(cfg.timeout));
        let page = tab.navigate_to(&url)?.wait_until_navigated()?;
        page.wait_for_element("#content-has-all-loaded-for-mdbook-pdf-generation")?;

        // Accept the Google Analytics cookie.
        if page.find_element("a.cookieBarConsentButton").is_ok() {
            page.evaluate(
                "document.querySelector('a.cookieBarConsentButton').click()",
                false,
            )?;
            println!("The book you built uses cookies from Google to deliver and enhance the quality of its services and to analyze traffic.");
            println!("Learn more at: https://policies.google.com/technologies/cookies");
        };

        // Find the theme and click it to change the theme.
        if !cloned_cfg.theme.is_empty() {
            match tab.find_element(&format!("button.theme#{}", cloned_cfg.theme.to_lowercase())) {
                Ok(_) => {
                    tab.evaluate(
                        &format!(
                            "document.querySelector('button.theme#{}').click()",
                            cloned_cfg.theme.to_lowercase()
                        ),
                        false,
                    )?;
                }
                Err(_) => println!(
                    "Unable to find theme {}, return to default one.",
                    cloned_cfg.theme.to_lowercase()
                ),
            };
        }

        // Generate the PDF.
        let generated_pdf = match page.print_to_pdf(Some(pdf_opts)) {
            Ok(output) => output,
            Err(e) => {
                if time == cloned_cfg.trying_times {
                    panic!("{}, so PDF generation failed!", e);
                }
                println!("{}, retrying after {} seconds...", e, time);
                thread::sleep(Duration::from_secs(time));
                continue;
            }
        };

        // Write the PDF to the destination.
        let generated_pdf_path = ctx
            .destination
            .join("output.pdf")
            .to_str()
            .unwrap()
            .to_owned();
        fs::write(generated_pdf_path.clone(), generated_pdf)?;
        println!("PDF successfully generated at: {}", generated_pdf_path);
        break;
    }

    Ok(())
}

#[macro_use]
extern crate serde_derive;

/**
 * Refer to https://docs.rs/headless_chrome/latest/headless_chrome/protocol/page/struct.PrintToPdfOptions.html
 * for member types of the PrintToPdfOptions
 */

#[derive(Deserialize, Debug, Clone)]
#[serde(default, rename_all = "kebab-case")]
pub struct PrintOptions {
    pub trying_times: u64,
    pub timeout: u64,
    pub browser_binary_path: String,
    pub static_site_url: String,
    pub landscape: bool,
    pub display_header_footer: bool,
    pub print_background: bool,
    pub theme: String,
    pub scale: f64,
    pub paper_width: f64,
    pub paper_height: f64,
    pub margin_top: f64,
    pub margin_bottom: f64,
    pub margin_left: f64,
    pub margin_right: f64,
    pub page_ranges: String,
    pub ignore_invalid_page_ranges: bool,
    pub header_template: String,
    pub footer_template: String,
    pub prefer_css_page_size: bool,
}

/**
 * Refer to https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-printToPDF
 * for the default values and meanings of the params for Page.PrintToPDF
 */

impl Default for PrintOptions {
    fn default() -> Self {
        PrintOptions {
            trying_times: 1u64,
            timeout: 600u64,
            browser_binary_path: "".to_string(),
            static_site_url: "".to_string(),
            landscape: false,
            display_header_footer: false,
            print_background: false,
            theme: "".to_string(),
            scale: 1_f64,
            paper_width: 8.5_f64,
            paper_height: 11_f64,
            margin_top: 1_f64,
            margin_bottom: 1_f64,
            margin_left: 1_f64,
            margin_right: 1_f64,
            page_ranges: "".to_string(),
            ignore_invalid_page_ranges: false,
            header_template: "".to_string(),
            footer_template: "".to_string(),
            prefer_css_page_size: false,
        }
    }
}
