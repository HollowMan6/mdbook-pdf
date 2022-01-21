/**
 * mdbook-pdf
 * Copyright (C) 2022 Hollow Man
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
use mdbook::renderer::RenderContext;
use std::{ffi::OsStr, fs, io, path::PathBuf, thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Receives the data passed to the program via mdbook
    let mut stdin = io::stdin();

    // Get the configs
    let ctx = RenderContext::from_json(&mut stdin).unwrap();
    let cfg: PrintOptions = ctx
        .config
        .get_deserialized_opt("output.pdf")
        .unwrap()
        .unwrap();

    // Used to relieve errors related to timeouts, but now for
    // the patches in my fork, it's not likely to be needed
    let trying_times = cfg.trying_times + 1;

    println!("Generating PDF, please be patient...");

    for time in 1..trying_times {
        let url = format!(
            "file://{}",
            ctx.destination
                .parent()
                .unwrap()
                .join("html")
                .join("print.html")
                .to_str()
                .unwrap()
        );

        let cloned_cfg = cfg.clone();
        let browser_binary = if cloned_cfg.browser_binary_path.is_empty() {
            None
        } else {
            Some(PathBuf::from(cloned_cfg.browser_binary_path))
        };

        let launch_opts = LaunchOptionsBuilder::default()
            .headless(true)
            .sandbox(false)
            .ignore_certificate_errors(false)
            .idle_browser_timeout(Duration::from_secs(u64::max_value()))
            .path(browser_binary)
            .args(vec![
                OsStr::new("--unlimited-storage"),
                OsStr::new("--disable-web-security"),
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
        let tab = browser.wait_for_initial_tab()?;
        let page = tab.navigate_to(&url)?.wait_until_navigated()?;

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
        fs::write(
            ctx.destination.join("output.pdf").to_str().unwrap(),
            &generated_pdf,
        )?;
        println!(
            "PDF successfully generated at: {}",
            ctx.destination.join("output.pdf").to_str().unwrap()
        );
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
    pub browser_binary_path: String,
    pub landscape: bool,
    pub display_header_footer: bool,
    pub print_background: bool,
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
            browser_binary_path: "".to_string(),
            landscape: false,
            display_header_footer: false,
            print_background: false,
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
