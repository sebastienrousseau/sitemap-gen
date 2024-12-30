// Copyright © 2025 Sitemap Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Provides core sitemap functionality and data structures.
//!
//! This module implements the main sitemap generation functionality according to the [Sitemaps XML format](https://www.sitemaps.org/protocol.html) specification.

use crate::config::{MAX_SITEMAP_SIZE, MAX_URLS, SITEMAP_XMLNS};
use crate::error::{SitemapError, SitemapResult};
use dtt::datetime::DateTime;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use url::Url;
use xml::writer::{EventWriter, XmlEvent};

lazy_static! {
    static ref DATE_REGEX: Regex =
        Regex::new(r"(\d{2}) (\w{3}) (\d{4})")
            .expect("Invalid date regex pattern");
}

/// Represents the data for a sitemap URL entry.
///
/// This struct contains all required fields for a sitemap URL entry according to the
/// [Sitemaps XML format](https://www.sitemaps.org/protocol.html).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiteMapData {
    /// How frequently the page is likely to change.
    /// This value provides a hint to search engines about the page's update frequency.
    pub changefreq: ChangeFreq,

    /// The date of last modification in YYYY-MM-DD format.
    /// Must be a valid date string in W3C Datetime format.
    pub lastmod: String,

    /// The canonical URL of the page.
    /// Must be a fully qualified URL that begins with http:// or https://.
    pub loc: Url,
}

impl SiteMapData {
    /// Creates a new `SiteMapData` instance with the provided values.
    ///
    /// # Arguments
    ///
    /// * `loc` - The URL of the page
    /// * `lastmod` - The last modification date
    /// * `changefreq` - How frequently the page is expected to change
    ///
    /// # Returns
    ///
    /// A new `SiteMapData` instance
    #[must_use]
    pub const fn new(
        loc: Url,
        lastmod: String,
        changefreq: ChangeFreq,
    ) -> Self {
        Self {
            loc,
            lastmod,
            changefreq,
        }
    }
}

/// Represents the change frequency of a URL in the sitemap.
///
/// This enum is used to indicate how frequently the page is likely to change.
/// Search engines use this information when deciding how often to crawl the page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeFreq {
    /// The page is changed every time it's accessed
    Always,
    /// The page is changed every hour
    Hourly,
    /// The page is changed every day
    Daily,
    /// The page is changed every week
    Weekly,
    /// The page is changed every month
    Monthly,
    /// The page is changed every year
    Yearly,
    /// The page is archived and never expected to change
    Never,
}

impl ChangeFreq {
    /// Returns the string representation of the change frequency.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::Hourly => "hourly",
            Self::Daily => "daily",
            Self::Weekly => "weekly",
            Self::Monthly => "monthly",
            Self::Yearly => "yearly",
            Self::Never => "never",
        }
    }
}

impl FromStr for ChangeFreq {
    type Err = SitemapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "always" => Ok(Self::Always),
            "hourly" => Ok(Self::Hourly),
            "daily" => Ok(Self::Daily),
            "weekly" => Ok(Self::Weekly),
            "monthly" => Ok(Self::Monthly),
            "yearly" => Ok(Self::Yearly),
            "never" => Ok(Self::Never),
            _ => Err(SitemapError::InvalidChangeFreq(s.to_string())),
        }
    }
}

impl fmt::Display for ChangeFreq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Represents a complete sitemap containing URL entries.
#[derive(Debug, Default, Clone)]
pub struct Sitemap {
    entries: Vec<SiteMapData>,
}

impl Sitemap {
    /// Creates a new empty `Sitemap`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `Sitemap` with the specified capacity.
    ///
    /// The capacity will be capped at [`MAX_URLS`] to prevent excessive memory allocation.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The desired capacity for the sitemap
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity.min(MAX_URLS)),
        }
    }

    /// Returns the current number of entries in the sitemap.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Checks if the sitemap is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Adds a new entry to the sitemap.
    ///
    /// # Errors
    ///
    /// Returns an error if adding the entry would exceed [`MAX_URLS`].
    pub fn add_entry(
        &mut self,
        entry: SiteMapData,
    ) -> SitemapResult<()> {
        if self.entries.len() >= MAX_URLS {
            return Err(SitemapError::MaxUrlLimitExceeded(
                self.entries.len(),
            ));
        }
        self.entries.push(entry);
        Ok(())
    }

    /// Adds multiple entries to the sitemap.
    ///
    /// # Errors
    ///
    /// Returns an error if adding any entry would exceed [`MAX_URLS`].
    pub fn add_entries<I>(&mut self, entries: I) -> SitemapResult<()>
    where
        I: IntoIterator<Item = SiteMapData>,
    {
        for entry in entries {
            self.add_entry(entry)?;
        }
        Ok(())
    }

    /// Generates the XML representation of the sitemap.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - XML writing fails
    /// - The generated XML exceeds [`MAX_SITEMAP_SIZE`]
    /// - UTF-8 encoding fails
    pub fn to_xml(&self) -> SitemapResult<String> {
        let estimated_size = self.entries.len().saturating_mul(300);
        let mut output = Vec::with_capacity(estimated_size);
        let mut writer = EventWriter::new(&mut output);

        self.write_xml_header(&mut writer)?;

        for entry in &self.entries {
            self.write_entry(&mut writer, entry)?;
        }

        writer.write(XmlEvent::end_element())?;

        let xml = String::from_utf8(output)
            .map_err(SitemapError::EncodingError)?;

        if xml.len() > MAX_SITEMAP_SIZE {
            return Err(SitemapError::SitemapTooLarge);
        }

        Ok(xml)
    }

    fn write_xml_header(
        &self,
        writer: &mut EventWriter<&mut Vec<u8>>,
    ) -> SitemapResult<()> {
        writer.write(XmlEvent::StartDocument {
            version: xml::common::XmlVersion::Version10,
            encoding: Some("UTF-8"),
            standalone: None,
        })?;

        writer.write(
            XmlEvent::start_element("urlset").default_ns(SITEMAP_XMLNS),
        )?;
        Ok(())
    }

    fn write_entry(
        &self,
        writer: &mut EventWriter<&mut Vec<u8>>,
        entry: &SiteMapData,
    ) -> SitemapResult<()> {
        writer.write(XmlEvent::start_element("url"))?;
        self.write_element(writer, "loc", entry.loc.as_str())?;
        self.write_element(writer, "lastmod", &entry.lastmod)?;
        self.write_element(
            writer,
            "changefreq",
            entry.changefreq.as_str(),
        )?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }

    fn write_element(
        &self,
        writer: &mut EventWriter<&mut Vec<u8>>,
        name: &str,
        value: &str,
    ) -> SitemapResult<()> {
        writer.write(XmlEvent::start_element(name))?;
        writer.write(XmlEvent::characters(value))?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

/// Generates `SiteMapData` from metadata.
///
/// Creates a sitemap entry from a metadata hash map containing page information.
///
/// # Arguments
///
/// * `metadata` - A hashmap containing page metadata with the following keys:
///   * `last_build_date` - The date the page was last modified
///   * `changefreq` - How frequently the page changes (optional, defaults to "weekly")
///   * `permalink` - The URL of the page (required)
///
/// # Returns
///
/// Returns a `SiteMapData` instance or an error if required data is missing or invalid.
///
/// # Errors
///
/// Returns an error if:
/// - The permalink is missing
/// - The URL is invalid
/// - The change frequency is invalid
pub fn create_site_map_data(
    metadata: &HashMap<String, String>,
) -> SitemapResult<SiteMapData> {
    let lastmod = convert_date_format(
        metadata.get("last_build_date").unwrap_or(&String::new()),
    );

    let changefreq = metadata
        .get("changefreq")
        .map(|s| s.parse())
        .transpose()?
        .unwrap_or(ChangeFreq::Weekly);

    let loc = metadata.get("permalink").ok_or_else(|| {
        SitemapError::CustomError(
            "Missing permalink in metadata".to_string(),
        )
    })?;
    let loc = Url::parse(loc).map_err(SitemapError::UrlError)?;

    Ok(SiteMapData::new(loc, lastmod, changefreq))
}

/// Converts date strings from various formats to "YYYY-MM-DD".
///
/// Supports conversion from multiple date formats:
/// - "DD MMM YYYY" (e.g., "20 May 2023")
/// - W3C Datetime format
/// - Any format supported by the `DateTime` parser
///
/// # Arguments
///
/// * `input` - A string slice representing the input date
///
/// # Returns
///
/// A string in "YYYY-MM-DD" format, or the original input if conversion fails
#[must_use]
pub fn convert_date_format(input: &str) -> String {
    if let Some(caps) = DATE_REGEX.captures(input) {
        let day = caps.get(1).map_or("", |m| m.as_str());
        let month = caps.get(2).map_or("", |m| m.as_str());
        let year = caps.get(3).map_or("", |m| m.as_str());

        let month_num = match month.to_lowercase().as_str() {
            "jan" => "01",
            "feb" => "02",
            "mar" => "03",
            "apr" => "04",
            "may" => "05",
            "jun" => "06",
            "jul" => "07",
            "aug" => "08",
            "sep" => "09",
            "oct" => "10",
            "nov" => "11",
            "dec" => "12",
            _ => return input.to_string(),
        };

        return format!("{year}-{month_num}-{day}");
    }

    DateTime::parse(input)
        .and_then(|dt| dt.format("[year]-[month]-[day]"))
        .unwrap_or_else(|_| input.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::SitemapError;

    // ---------------------------
    //  SiteMapData-specific Tests
    // ---------------------------
    mod site_map_data_tests {
        use super::*;

        /// Verifies that `SiteMapData::new` initializes all fields correctly.
        #[test]
        fn test_site_map_data_new() {
            let loc = Url::parse("https://example.net").unwrap();
            let lastmod = "2026-01-01".to_string();
            let changefreq = ChangeFreq::Hourly;

            let data = SiteMapData::new(
                loc.clone(),
                lastmod.clone(),
                changefreq,
            );

            assert_eq!(data.loc, loc);
            assert_eq!(data.lastmod, lastmod);
            assert_eq!(data.changefreq, changefreq);
        }
    }

    // ---------------------------
    //  create_site_map_data Tests
    // ---------------------------
    mod create_site_map_data_tests {
        use super::*;

        /// Checks that `create_site_map_data` builds the correct `SiteMapData` from metadata.
        #[test]
        fn test_create_site_map_data() -> SitemapResult<()> {
            let mut metadata = HashMap::new();
            let _ = metadata.insert(
                "last_build_date".to_string(),
                "20 May 2023".to_string(),
            );
            let _ = metadata
                .insert("changefreq".to_string(), "weekly".to_string());
            let _ = metadata.insert(
                "permalink".to_string(),
                "https://example.com".to_string(),
            );

            let site_map_data = create_site_map_data(&metadata)?;

            assert_eq!(site_map_data.lastmod, "2023-05-20");
            assert_eq!(site_map_data.changefreq, ChangeFreq::Weekly);
            assert_eq!(
                site_map_data.loc,
                Url::parse("https://example.com")?
            );

            Ok(())
        }

        /// Ensures an error is raised if the `permalink` field is missing.
        #[test]
        fn test_create_site_map_data_missing_permalink() {
            let mut metadata = HashMap::new();
            // Missing "permalink" key
            let _ = metadata.insert(
                "last_build_date".to_string(),
                "20 May 2023".to_string(),
            );
            let _ = metadata
                .insert("changefreq".to_string(), "weekly".to_string());

            let result = create_site_map_data(&metadata);
            assert!(
                matches!(result, Err(SitemapError::CustomError(msg)) if msg.contains("Missing permalink")),
                "Expected an error about missing permalink"
            );
        }

        /// Ensures an error is raised if the `permalink` is not a valid URL.
        #[test]
        fn test_create_site_map_data_invalid_permalink() {
            let mut metadata = HashMap::new();
            let _ = metadata.insert(
                "permalink".to_string(),
                "not-a-valid-url".to_string(),
            );
            // "last_build_date" omitted for brevity

            let result = create_site_map_data(&metadata);
            assert!(
                matches!(result, Err(SitemapError::UrlError(_))),
                "Expected a URL parsing error"
            );
        }

        /// Ensures an error is raised if `changefreq` is not recognized.
        #[test]
        fn test_create_site_map_data_invalid_changefreq() {
            let mut metadata = HashMap::new();
            let _ = metadata.insert(
                "permalink".to_string(),
                "https://example.com".to_string(),
            );
            let _ = metadata.insert(
                "changefreq".to_string(),
                "very-often".to_string(),
            );

            let result = create_site_map_data(&metadata);
            assert!(
                matches!(result, Err(SitemapError::InvalidChangeFreq(freq)) if freq == "very-often"),
                "Expected an InvalidChangeFreq error for unrecognized freq"
            );
        }
    }

    // ----------------------
    //  ChangeFreq Tests
    // ----------------------
    mod change_freq_tests {
        use super::*;

        /// Verifies that `ChangeFreq::as_str()` returns the correct string for each variant.
        #[test]
        fn test_change_freq_as_str() {
            assert_eq!(ChangeFreq::Always.as_str(), "always");
            assert_eq!(ChangeFreq::Hourly.as_str(), "hourly");
            assert_eq!(ChangeFreq::Daily.as_str(), "daily");
            assert_eq!(ChangeFreq::Weekly.as_str(), "weekly");
            assert_eq!(ChangeFreq::Monthly.as_str(), "monthly");
            assert_eq!(ChangeFreq::Yearly.as_str(), "yearly");
            assert_eq!(ChangeFreq::Never.as_str(), "never");
        }

        /// Checks the `Display` implementation of a few `ChangeFreq` variants.
        #[test]
        fn test_change_freq_display() {
            assert_eq!(ChangeFreq::Daily.to_string(), "daily");
            assert_eq!(ChangeFreq::Weekly.to_string(), "weekly");
            assert_eq!(ChangeFreq::Monthly.to_string(), "monthly");
        }

        /// Ensures `from_str` can parse valid variants and fails on invalid input.
        #[test]
        fn test_change_freq_from_str() {
            assert_eq!(
                "daily".parse::<ChangeFreq>().unwrap(),
                ChangeFreq::Daily
            );
            assert_eq!(
                "WEEKLY".parse::<ChangeFreq>().unwrap(),
                ChangeFreq::Weekly
            );
            assert!("invalid".parse::<ChangeFreq>().is_err());
        }
    }

    // --------------------------
    //  convert_date_format Tests
    // --------------------------
    mod date_format_tests {
        use super::*;

        /// Checks that common date formats convert correctly (or remain unchanged if invalid).
        #[test]
        fn test_convert_date_format() {
            assert_eq!(
                convert_date_format("20 May 2023"),
                "2023-05-20"
            );
            assert_eq!(convert_date_format("2023-05-20"), "2023-05-20");
            assert_eq!(
                convert_date_format("Invalid Date"),
                "Invalid Date"
            );
        }

        /// Covers edge cases, including empty strings, partially valid strings, etc.
        #[test]
        fn test_convert_date_format_edge_cases() {
            assert_eq!(convert_date_format(""), "");
            assert_eq!(
                convert_date_format("Invalid Date"),
                "Invalid Date"
            );
            assert_eq!(
                convert_date_format("32 Jan 2023"),
                "2023-01-32"
            );
            assert_eq!(
                convert_date_format("01 Foo 2023"),
                "01 Foo 2023"
            );
        }
    }

    // ----------------------
    //  Sitemap Tests
    // ----------------------
    mod sitemap_tests {
        use super::*;

        /// Ensures a `Sitemap` created via `Sitemap::new()` is empty and has length 0.
        #[test]
        fn test_sitemap_new_is_empty() {
            let sitemap = Sitemap::new();
            assert_eq!(
                sitemap.len(),
                0,
                "Newly created sitemap should have length 0"
            );
            assert!(
                sitemap.is_empty(),
                "Newly created sitemap should be empty"
            );
        }

        /// Demonstrates `Sitemap::with_capacity` respects capacity up to `MAX_URLS`.
        #[test]
        fn test_sitemap_with_capacity() {
            let sitemap = Sitemap::with_capacity(100);
            assert!(sitemap.entries.capacity() >= 100);
            assert!(sitemap.entries.capacity() <= MAX_URLS);
        }

        /// Verifies behavior of `Sitemap::len()` and `Sitemap::is_empty()` after adding an entry.
        #[test]
        fn test_sitemap_len_and_is_empty_with_entries(
        ) -> SitemapResult<()> {
            let mut sitemap = Sitemap::new();
            let entry = SiteMapData::new(
                Url::parse("https://example.com")?,
                "2023-05-20".to_string(),
                ChangeFreq::Weekly,
            );
            sitemap.add_entry(entry)?;

            assert_eq!(
                sitemap.len(),
                1,
                "Sitemap should have length 1 after adding one entry"
            );
            assert!(
                !sitemap.is_empty(),
                "Sitemap should not be empty after adding an entry"
            );
            Ok(())
        }

        /// Tests adding a single entry to the sitemap.
        #[test]
        fn test_add_entry_single() -> SitemapResult<()> {
            let mut sitemap = Sitemap::new();
            let entry = SiteMapData::new(
                Url::parse("https://example.org")?,
                "2025-12-30".to_string(),
                ChangeFreq::Daily,
            );
            sitemap.add_entry(entry.clone())?;

            assert_eq!(sitemap.len(), 1);
            assert_eq!(sitemap.entries[0], entry);
            Ok(())
        }

        /// Tests bulk addition of entries using `Sitemap::add_entries`.
        #[test]
        fn test_add_entries_bulk() -> SitemapResult<()> {
            let mut sitemap = Sitemap::new();
            let entries = vec![
                SiteMapData::new(
                    Url::parse("https://example.com/1")?,
                    "2024-01-01".to_string(),
                    ChangeFreq::Daily,
                ),
                SiteMapData::new(
                    Url::parse("https://example.com/2")?,
                    "2024-01-02".to_string(),
                    ChangeFreq::Weekly,
                ),
            ];

            sitemap.add_entries(entries)?;
            assert_eq!(sitemap.len(), 2);
            Ok(())
        }

        /// Validates the XML serialization logic of `Sitemap::to_xml()`.
        #[test]
        fn test_sitemap_to_xml() -> SitemapResult<()> {
            let mut sitemap = Sitemap::new();
            sitemap.add_entry(SiteMapData::new(
                Url::parse("https://example.com")?,
                "2023-05-20".to_string(),
                ChangeFreq::Weekly,
            ))?;

            let xml = sitemap.to_xml()?;

            assert!(xml.contains("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">"));
            assert!(xml.contains("<url>"));
            assert!(xml.contains("<loc>https://example.com/</loc>"));
            assert!(xml.contains("<lastmod>2023-05-20</lastmod>"));
            assert!(xml.contains("<changefreq>weekly</changefreq>"));
            Ok(())
        }

        /// Ensures that adding more URLs than allowed triggers `SitemapError::MaxUrlLimitExceeded`.
        #[test]
        fn test_sitemap_size_limit() -> SitemapResult<()> {
            let mut sitemap = Sitemap::with_capacity(MAX_URLS);

            for i in 0..MAX_URLS {
                sitemap.add_entry(SiteMapData::new(
                    Url::parse(&format!("https://example.com/{i}"))?,
                    "2023-05-20".to_string(),
                    ChangeFreq::Weekly,
                ))?;
            }

            let result = sitemap.add_entry(SiteMapData::new(
                Url::parse("https://example.com/toomany")?,
                "2023-05-20".to_string(),
                ChangeFreq::Weekly,
            ));

            assert!(
                matches!(
                    result,
                    Err(SitemapError::MaxUrlLimitExceeded(_))
                ),
                "Expected an error when exceeding max URLs"
            );
            Ok(())
        }

        /// Tests that generating an extremely large XML triggers `SitemapError::SitemapTooLarge`.
        #[test]
        fn test_sitemap_too_large_error() {
            let mut sitemap = Sitemap::new();

            // Construct a large string that should exceed MAX_SITEMAP_SIZE when serialized.
            let huge_loc_string = format!(
                "https://example.com/{}",
                "a".repeat(MAX_SITEMAP_SIZE + 10) // Enough to push over the limit
            );

            let entry = SiteMapData {
                loc: Url::parse(&huge_loc_string).unwrap(),
                lastmod: "2023-05-20".to_string(),
                changefreq: ChangeFreq::Weekly,
            };

            // Add a single entry that pushes us over the size threshold.
            sitemap.add_entry(entry).unwrap();

            let result = sitemap.to_xml();
            assert!(
                matches!(result, Err(SitemapError::SitemapTooLarge)),
                "Expected a SitemapTooLarge error"
            );
        }

        /// Ensures that concurrent writes don't interfere with each other.
        #[test]
        fn test_concurrent_sitemap_read() -> SitemapResult<()> {
            use std::sync::Arc;
            use std::thread;

            let sitemap = Arc::new(Sitemap::new());
            let mut handles = Vec::new();

            for _ in 0..10 {
                let sitemap = Arc::clone(&sitemap);
                handles.push(thread::spawn(move || {
                    assert_eq!(sitemap.len(), 0);
                    assert!(sitemap.is_empty());
                }));
            }

            for handle in handles {
                handle.join().map_err(|_| {
                    SitemapError::CustomError(
                        "Thread panicked during read test".to_string(),
                    )
                })?;
            }

            Ok(())
        }
    }
}
