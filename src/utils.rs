use crate::{
    ChangeFreq, SiteMapData, Sitemap, SitemapError, SitemapResult,
};
use clap::{Arg, ArgAction, Command};
use dtt::{datetime::DateTime, dtt_now};
use indicatif::{ProgressBar, ProgressStyle};
use log::{info, warn};
use std::io::BufRead;
use std::io::Write;
use std::{collections::HashSet, fs::File, io};
use url::Url;

/// Maximum number of URLs allowed in a single sitemap.
pub const MAX_URLS: usize = 50000;

/// Default change frequency for URLs.
pub const DEFAULT_CHANGE_FREQ: &str = "weekly";

/// Creates the command-line interface for the application.
///
/// This function defines all the possible arguments and options
/// for the sitemap generator CLI.
pub fn create_cli() -> Command {
    Command::new("Sitemap Generator")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Your Name <your.email@example.com>")
        .about("Generates XML sitemaps")
        .subcommand(
            Command::new("generate")
                .about("Generates a sitemap")
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("Sets the output file")
                        .required(true),
                )
                .arg(
                    Arg::new("url")
                        .short('u')
                        .long("url")
                        .value_name("URL")
                        .help("Adds a URL to the sitemap")
                        .action(ArgAction::Append)
                        .conflicts_with("input"),
                )
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .value_name("FILE")
                        .help("Read URLs from a file")
                        .conflicts_with("url"),
                )
                .arg(
                    Arg::new("changefreq")
                        .short('c')
                        .long("changefreq")
                        .value_name("FREQ")
                        .help("Sets the change frequency for all URLs")
                        .default_value(DEFAULT_CHANGE_FREQ),
                )
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .help("Enable verbose output")
                        .action(ArgAction::SetTrue),
                ),
        )
}

/// Generates a sitemap based on the provided command-line arguments.
///
/// This function handles the core logic of sitemap generation, including
/// reading URLs, creating sitemap entries, and writing the output file.
///
/// # Arguments
///
/// * `matches` - The matches from the command-line argument parsing
///
/// # Errors
///
/// This function will return an error if:
/// - There are issues reading input files
/// - URL parsing fails
/// - The number of URLs exceeds the maximum limit
/// - Sitemap generation fails
/// - Writing output files fails
pub fn generate_sitemap(
    matches: &clap::ArgMatches,
) -> SitemapResult<()> {
    let output_file = matches.get_one::<String>("output").unwrap();
    let verbose = matches.get_flag("verbose");

    let urls = if let Some(input_file) =
        matches.get_one::<String>("input")
    {
        read_urls_from_file(input_file)?
    } else if let Some(url_values) = matches.get_many::<String>("url") {
        url_values
            .map(|s| Url::parse(s).map_err(SitemapError::UrlError))
            .collect::<Result<Vec<Url>, SitemapError>>()?
    } else {
        return Err(SitemapError::CustomError(
            "No URLs provided. Use either -u or -i option.".to_string(),
        ));
    };

    let urls = normalize_urls(urls);

    if urls.len() > MAX_URLS {
        return Err(SitemapError::MaxUrlLimitExceeded(urls.len()));
    }

    let default_change_freq = DEFAULT_CHANGE_FREQ.to_string();
    let changefreq_str = matches
        .get_one::<String>("changefreq")
        .unwrap_or(&default_change_freq);
    let changefreq = changefreq_str.parse::<ChangeFreq>()?;

    let mut sitemap = Sitemap::new();

    let progress_bar = if verbose {
        let pb = ProgressBar::new(urls.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("##-"),
        );
        Some(pb)
    } else {
        None
    };

    for (index, url) in urls.iter().enumerate() {
        if let Some(pb) = &progress_bar {
            pb.set_message(format!("Processing: {}", url));
            pb.inc(1);
        } else if verbose {
            info!(
                "Processing URL {}/{}: {}",
                index + 1,
                urls.len(),
                url
            );
        }

        let entry = SiteMapData {
            loc: url.clone(),
            lastmod: format_date(dtt_now!()),
            changefreq,
        };
        sitemap.add_entry(entry)?;
    }

    if let Some(pb) = progress_bar {
        pb.finish_with_message("Sitemap generation complete");
    }

    if verbose {
        info!("Writing sitemap to file...");
    }

    let xml = sitemap.to_xml()?;
    write_output(&xml, output_file)?;

    info!("Sitemap generated successfully: {}", output_file);
    Ok(())
}

/// Reads URLs from a file, one URL per line.
///
/// # Arguments
///
/// * `filename` - The name of the file to read URLs from
///
/// # Errors
///
/// This function will return an error if:
/// - The file cannot be opened
/// - There are issues reading lines from the file
/// - Any of the URLs in the file are invalid
pub fn read_urls_from_file(filename: &str) -> SitemapResult<Vec<Url>> {
    let file = File::open(filename).map_err(SitemapError::IoError)?;
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let line = line.ok()?;
            if line.trim().is_empty() {
                return None;
            }
            match Url::parse(&line) {
                Ok(url) => Some(Ok(url)),
                Err(e) => {
                    warn!(
                        "Invalid URL on line {}: '{}'. Error: {}",
                        index + 1,
                        line,
                        e
                    );
                    Some(Err(SitemapError::UrlError(e)))
                }
            }
        })
        .collect()
}

/// Normalizes a list of URLs to avoid duplicates.
///
/// This function removes URL fragments and ensures each URL ends with a trailing slash
/// if it doesn't have a path or if the path is just "/".
/// It also logs a warning if duplicate URLs are found after normalization.
/// Invalid URLs (those not using http or https schemes) are filtered out.
///
/// # Arguments
///
/// * `urls` - A vector of URLs to normalize
///
/// # Returns
///
/// A vector of normalized unique URLs
pub fn normalize_urls(urls: Vec<Url>) -> Vec<Url> {
    let mut normalized = HashSet::new();
    for mut url in urls {
        if !is_valid_url(&url) {
            warn!("Invalid URL scheme: {}", url);
            continue;
        }
        url.set_fragment(None);
        if url.path().is_empty() || url.path() == "/" {
            url.set_path("/");
        }
        if !normalized.insert(url.clone()) {
            warn!("Duplicate URL found after normalization: {}", url);
        }
    }
    normalized.into_iter().collect()
}

/// Checks if a URL is valid for inclusion in the sitemap.
///
/// This function checks if the URL uses either the HTTP or HTTPS scheme.
///
/// # Arguments
///
/// * `url` - The URL to validate
///
/// # Returns
///
/// `true` if the URL is valid, `false` otherwise
pub fn is_valid_url(url: &Url) -> bool {
    matches!(url.scheme(), "http" | "https")
}

/// Writes the sitemap XML to an output file.
///
/// # Arguments
///
/// * `xml` - The XML content to write
/// * `output_file` - The name of the output file
///
/// # Errors
///
/// This function will return an error if:
/// - The output file cannot be created
/// - There are issues writing to the file
pub fn write_output(xml: &str, output_file: &str) -> SitemapResult<()> {
    let mut file =
        File::create(output_file).map_err(SitemapError::IoError)?;
    file.write_all(xml.as_bytes())
        .map_err(SitemapError::IoError)?;
    Ok(())
}

/// Formats a DateTime object into a string suitable for sitemap use.
///
/// # Arguments
///
/// * `dt` - The DateTime object to format
///
/// # Returns
///
/// A string representation of the date in YYYY-MM-DD format
pub fn format_date(dt: DateTime) -> String {
    dt.format("[year]-[month]-[day]")
        .unwrap_or_else(|_| "".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{
        is_valid_url, normalize_urls, read_urls_from_file,
    };
    use crate::SitemapError;
    use std::io::Write;
    use tempfile::NamedTempFile;
    use url::Url;

    #[test]
    fn test_read_urls_from_file() -> SitemapResult<()> {
        let mut temp_file =
            NamedTempFile::new().map_err(SitemapError::IoError)?;
        writeln!(temp_file, "https://example.com")
            .map_err(SitemapError::IoError)?;
        writeln!(temp_file, "https://example.org")
            .map_err(SitemapError::IoError)?;
        writeln!(temp_file).map_err(SitemapError::IoError)?;
        writeln!(temp_file, "https://example.net")
            .map_err(SitemapError::IoError)?;

        let urls =
            read_urls_from_file(temp_file.path().to_str().unwrap())?;
        assert_eq!(urls.len(), 3);
        assert_eq!(urls[0].as_str(), "https://example.com/");
        assert_eq!(urls[1].as_str(), "https://example.org/");
        assert_eq!(urls[2].as_str(), "https://example.net/");

        Ok(())
    }

    #[test]
    fn test_invalid_url_in_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "invalid_url").unwrap();

        let result =
            read_urls_from_file(temp_file.path().to_str().unwrap());
        assert!(result.is_err(), "Expected an error for invalid URL");
    }

    #[test]
    fn test_normalize_urls() {
        let urls = vec![
            Url::parse("http://example.com").unwrap(),
            Url::parse("http://example.com/").unwrap(),
            Url::parse("http://example.com/page#fragment").unwrap(),
            Url::parse("http://example.org").unwrap(),
            Url::parse("ftp://example.net").unwrap(), // This should be filtered out
        ];

        let normalized = normalize_urls(urls);
        assert_eq!(normalized.len(), 3);
        assert!(normalized
            .contains(&Url::parse("http://example.com/").unwrap()));
        assert!(normalized
            .contains(&Url::parse("http://example.org/").unwrap()));
        assert!(normalized
            .contains(&Url::parse("http://example.com/page").unwrap()));
        assert!(!normalized
            .contains(&Url::parse("ftp://example.net").unwrap()));
    }

    #[test]
    fn test_is_valid_url() {
        assert!(is_valid_url(
            &Url::parse("http://example.com").unwrap()
        ));
        assert!(is_valid_url(
            &Url::parse("https://example.com").unwrap()
        ));
        assert!(!is_valid_url(
            &Url::parse("ftp://example.com").unwrap()
        ));
    }
}
