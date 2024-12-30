// Copyright © 2025 Sitemap Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Sitemap Generator CLI
//!
//! This command-line application allows users to generate XML sitemaps.
//! It provides functionality to add URLs and specify their change frequency.
//!
//! ## Features:
//! - Generate XML sitemaps with multiple URLs
//! - Specify change frequency for URLs
//! - Read URLs from command line or input file
//! - Verbose mode for detailed output
//! - URL count limit to comply with sitemap standards
//! - URL normalization to avoid duplicates
//! - Progress indicator for large input files
//!
//! ## Example usage:
//! ```bash
//! sitemap-gen generate -o output.xml -u "http://example.com" -c weekly
//! sitemap-gen generate -o output.xml -i urls.txt -c daily -v
//! ```

//! # Sitemap Generator CLI
//!
//! This command-line application allows users to generate XML sitemaps.
//! It provides functionality to add URLs and specify their change frequency.
//!
//! ## Features:
//! - Generate XML sitemaps with multiple URLs
//! - Specify change frequency for URLs
//! - Read URLs from command line or input file
//! - Verbose mode for detailed output
//! - URL count limit to comply with sitemap standards
//! - URL normalization to avoid duplicates
//! - Progress indicator for large input files
//!
//! ## Example usage:
//! ```bash
//! sitemap-gen generate -o output.xml -u "http://example.com" -c weekly
//! sitemap-gen generate -o output.xml -i urls.txt -c daily -v
//! ```

use sitemap_gen::utils::{create_cli, generate_sitemap};
use sitemap_gen::SitemapResult;

/// The main entry point for the Sitemap Generator CLI.
///
/// This function sets up the command-line interface, parses arguments,
/// and orchestrates the sitemap generation process.
///
/// # Errors
///
/// This function will return an error if:
/// - There are issues parsing command-line arguments
/// - There are problems reading input files
/// - Sitemap generation fails
/// - Writing output files fails
fn main() -> SitemapResult<()> {
    env_logger::init();

    let matches = create_cli().get_matches();

    if let Some(matches) = matches.subcommand_matches("generate") {
        generate_sitemap(matches)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::process::Command;

    #[test]
    fn test_generate_sitemap_with_single_url() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("generate")
            .arg("-o")
            .arg("test_output.xml")
            .arg("-u")
            .arg("http://example.com")
            .arg("-c")
            .arg("weekly")
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        assert!(
            fs::metadata("test_output.xml").is_ok(),
            "Output file not created"
        );
    }

    #[test]
    fn test_generate_sitemap_with_invalid_url() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("generate")
            .arg("-o")
            .arg("test_output.xml")
            .arg("-u")
            .arg("invalid-url")
            .output()
            .expect("Failed to execute command");

        assert!(
            !output.status.success(),
            "Command should fail with invalid URL"
        );

        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("stderr: {}", stderr); // Debugging output

        // Assert against the actual error message
        assert!(
            stderr.contains("UrlError(RelativeUrlWithoutBase)"),
            "Expected error about relative URL without base"
        );
    }

    #[test]
    fn test_generate_sitemap_with_input_file() {
        fs::write(
            "test_urls.txt",
            "http://example.com\nhttp://example.org",
        )
        .expect("Failed to write test file");

        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("generate")
            .arg("-o")
            .arg("test_output.xml")
            .arg("-i")
            .arg("test_urls.txt")
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        assert!(
            fs::metadata("test_output.xml").is_ok(),
            "Output file not created"
        );
    }
}
