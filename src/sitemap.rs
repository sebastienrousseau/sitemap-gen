// src/sitemap.rs

use crate::error::{SitemapError, SitemapResult};
use dtt::datetime::DateTime;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use url::Url;
use xml::writer::{EventWriter, XmlEvent};

/// Maximum number of URLs allowed in a sitemap.
const MAX_URLS: usize = 50_000;

/// Represents the data for a sitemap entry.
#[derive(Debug, Clone, PartialEq)]
pub struct SiteMapData {
    /// The change frequency of the URL.
    pub changefreq: ChangeFreq,
    /// The last modification date of the URL in YYYY-MM-DD format.
    pub lastmod: String,
    /// The location (URL) of the page.
    pub loc: Url,
}

/// Represents the change frequency of a URL in the sitemap.
///
/// This enum is used to indicate how frequently the page is likely to change.
/// Search engines use this information when deciding how often to crawl the page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeFreq {
    /// The page is changed every time it's accessed.
    Always,
    /// The page is changed every hour.
    Hourly,
    /// The page is changed every day.
    Daily,
    /// The page is changed every week.
    Weekly,
    /// The page is changed every month.
    Monthly,
    /// The page is changed every year.
    Yearly,
    /// The page is archived and never expected to change.
    Never,
}

impl ChangeFreq {
    /// Returns the string representation of the change frequency.
    pub fn as_str(&self) -> &'static str {
        match self {
            ChangeFreq::Always => "always",
            ChangeFreq::Hourly => "hourly",
            ChangeFreq::Daily => "daily",
            ChangeFreq::Weekly => "weekly",
            ChangeFreq::Monthly => "monthly",
            ChangeFreq::Yearly => "yearly",
            ChangeFreq::Never => "never",
        }
    }
}

impl FromStr for ChangeFreq {
    type Err = SitemapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "always" => Ok(ChangeFreq::Always),
            "hourly" => Ok(ChangeFreq::Hourly),
            "daily" => Ok(ChangeFreq::Daily),
            "weekly" => Ok(ChangeFreq::Weekly),
            "monthly" => Ok(ChangeFreq::Monthly),
            "yearly" => Ok(ChangeFreq::Yearly),
            "never" => Ok(ChangeFreq::Never),
            _ => Err(SitemapError::InvalidChangeFreq(s.to_string())),
        }
    }
}

impl fmt::Display for ChangeFreq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ChangeFreq::Always => "always",
            ChangeFreq::Hourly => "hourly",
            ChangeFreq::Daily => "daily",
            ChangeFreq::Weekly => "weekly",
            ChangeFreq::Monthly => "monthly",
            ChangeFreq::Yearly => "yearly",
            ChangeFreq::Never => "never",
        };
        write!(f, "{}", s)
    }
}

/// Generates `SiteMapData` from metadata.
///
/// # Arguments
/// * `metadata` - A hashmap containing page metadata, including last build date, change frequency, and page location.
///
/// # Returns
/// A `SiteMapData` object populated with values from the metadata, or an error if the data is invalid.
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

    Ok(SiteMapData {
        changefreq,
        lastmod,
        loc,
    })
}

lazy_static! {
    static ref DATE_REGEX: Regex =
        Regex::new(r"(\d{2}) (\w{3}) (\d{4})").unwrap();
}

/// Converts date strings from various formats to "YYYY-MM-DD".
///
/// Supports conversion from "DD MMM YYYY" format and checks if input is already in target format.
///
/// # Arguments
/// * `input` - A string slice representing the input date.
///
/// # Returns
/// A string representing the date in "YYYY-MM-DD" format, or the original input if conversion is not applicable.
pub fn convert_date_format(input: &str) -> String {
    if let Some(caps) = DATE_REGEX.captures(input) {
        let day = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let month = caps.get(2).map(|m| m.as_str()).unwrap_or("");
        let year = caps.get(3).map(|m| m.as_str()).unwrap_or("");

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

        return format!("{}-{}-{}", year, month_num, day);
    }

    if let Ok(dt) = DateTime::parse(input) {
        if let Ok(formatted) = dt.format("[year]-[month]-[day]") {
            return formatted;
        }
    }

    input.to_string()
}

/// Represents a complete sitemap.
#[derive(Debug, Default, Clone)]
pub struct Sitemap {
    entries: Vec<SiteMapData>,
}

impl Sitemap {
    /// Creates a new empty `Sitemap`.
    pub fn new() -> Self {
        Sitemap {
            entries: Vec::new(),
        }
    }

    /// Entry count of the sitemap.
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Adds a new entry to the sitemap.
    ///
    /// # Arguments
    /// * `entry` - The `SiteMapData` entry to add to the sitemap.
    ///
    /// # Returns
    /// `Ok(())` if the entry was added successfully, or an error if the sitemap would exceed size limits.
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

    /// Returns the current number of entries in the sitemap.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Checks if the sitemap is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Generates the XML representation of the sitemap.
    ///
    /// # Returns
    /// A string containing the XML representation of the sitemap, or an error if generation fails.
    pub fn to_xml(&self) -> SitemapResult<String> {
        // Pre-allocate enough space in the Vec to avoid reallocations.
        let estimated_size = self.entries.len() * 300; // Rough estimate of average entry size in bytes
        let mut output = Vec::with_capacity(estimated_size);
        let mut writer = EventWriter::new(&mut output);

        writer.write(XmlEvent::StartDocument {
            version: xml::common::XmlVersion::Version10,
            encoding: Some("UTF-8"),
            standalone: None,
        })?;

        writer.write(XmlEvent::start_element("urlset").default_ns(
            "http://www.sitemaps.org/schemas/sitemap/0.9",
        ))?;

        for entry in &self.entries {
            // Start the <url> element
            writer.write(XmlEvent::start_element("url"))?;

            // <loc> entry
            writer.write(XmlEvent::start_element("loc"))?;
            writer.write(XmlEvent::characters(entry.loc.as_ref()))?;
            writer.write(XmlEvent::end_element())?;

            // <lastmod> entry
            writer.write(XmlEvent::start_element("lastmod"))?;
            writer.write(XmlEvent::characters(&entry.lastmod))?;
            writer.write(XmlEvent::end_element())?;

            // <changefreq> entry
            writer.write(XmlEvent::start_element("changefreq"))?;
            writer.write(XmlEvent::characters(
                entry.changefreq.as_str(),
            ))?;
            writer.write(XmlEvent::end_element())?;

            // End the <url> element
            writer.write(XmlEvent::end_element())?;
        }

        // Close the <urlset> element
        writer.write(XmlEvent::end_element())?;

        // Convert the output Vec<u8> directly into a string without intermediate allocations
        let xml = unsafe { String::from_utf8_unchecked(output) };

        // Check size before returning to ensure the sitemap isn't too large
        if xml.len() > 10 * 1024 * 1024 {
            return Err(SitemapError::SitemapTooLarge);
        }

        Ok(xml)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dtt::dtt_now;

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

    #[test]
    fn test_convert_date_format() {
        assert_eq!(convert_date_format("20 May 2023"), "2023-05-20");
        assert_eq!(convert_date_format("2023-05-20"), "2023-05-20");
        assert_eq!(convert_date_format("Invalid Date"), "Invalid Date");
    }

    #[test]
    fn test_sitemap_to_xml() -> SitemapResult<()> {
        let mut sitemap = Sitemap::new();
        sitemap.add_entry(SiteMapData {
            loc: Url::parse("https://example.com")?,
            lastmod: "2023-05-20".to_string(),
            changefreq: ChangeFreq::Weekly,
        })?;

        let xml = sitemap.to_xml()?;
        assert!(xml.contains("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">"));
        assert!(xml.contains("<url>"));
        assert!(xml.contains("<loc>https://example.com/</loc>"));
        assert!(xml.contains("<lastmod>2023-05-20</lastmod>"));
        assert!(xml.contains("<changefreq>weekly</changefreq>"));
        Ok(())
    }

    #[test]
    fn test_sitemap_size_limit() -> SitemapResult<()> {
        let mut sitemap = Sitemap::new();
        for i in 0..50_000 {
            sitemap.add_entry(SiteMapData {
                loc: Url::parse(&format!("https://example.com/{}", i))?,
                lastmod: "2023-05-20".to_string(),
                changefreq: ChangeFreq::Weekly,
            })?;
        }
        assert!(matches!(
            sitemap.add_entry(SiteMapData {
                loc: Url::parse("https://example.com/toomany")?,
                lastmod: "2023-05-20".to_string(),
                changefreq: ChangeFreq::Weekly,
            }),
            Err(SitemapError::MaxUrlLimitExceeded(_))
        ));
        Ok(())
    }

    #[test]
    fn test_dtt_now_macro() {
        let now = dtt_now!();
        assert!(now.year() >= 2023);
    }
    #[test]
    fn test_convert_date_format_edge_cases() {
        assert_eq!(convert_date_format(""), "");
        assert_eq!(convert_date_format("Invalid Date"), "Invalid Date");
        assert_eq!(convert_date_format("32 Jan 2023"), "2023-01-32");
        assert_eq!(convert_date_format("01 Foo 2023"), "01 Foo 2023");
    }
}
