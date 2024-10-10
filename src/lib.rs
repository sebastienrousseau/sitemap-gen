// src/lib.rs

#![doc = include_str!("../README.md")]
#![doc(
    html_favicon_url = "https://kura.pro/sitemap-gen/images/favicon.ico",
    html_logo_url = "https://kura.pro/sitemap-gen/images/logos/sitemap-gen.svg",
    html_root_url = "https://docs.rs/sitemap-gen"
)]
#![crate_name = "sitemap_gen"]
#![crate_type = "lib"]

//! A Rust library for generating and managing sitemaps.
//!
//! This crate provides functionality to create, modify, and serialize XML sitemaps according to the [Sitemaps XML format](https://www.sitemaps.org/protocol.html).
//! It includes support for handling various sitemap-specific data types and error conditions.

/// Contains error types specific to sitemap operations.
///
/// This module defines a comprehensive set of error types that can occur during
/// sitemap creation, modification, and serialization processes.
pub mod error;

/// Provides the core functionality for creating and managing sitemaps.
///
/// This module contains the main structures and functions for working with sitemaps,
/// including creating sitemap entries, setting change frequencies, and serializing to XML.
pub mod sitemap;

/// Utility functions and helper methods for sitemap operations.
pub mod utils;

// Re-exports
pub use error::SitemapError;
pub use sitemap::{
    convert_date_format, create_site_map_data, ChangeFreq, SiteMapData,
    Sitemap,
};

/// Result type alias for sitemap operations.
pub type SitemapResult<T> = Result<T, SitemapError>;

/// A prelude module for convenient importing of commonly used items.
pub mod prelude {
    pub use crate::error::SitemapError;
    pub use crate::sitemap::{ChangeFreq, SiteMapData, Sitemap};
    pub use crate::SitemapResult;
}

#[cfg(test)]
mod tests {
    use url::Url;

    use super::*;
    use crate::error::SitemapError;
    use crate::sitemap::{ChangeFreq, SiteMapData, Sitemap};
    use crate::SitemapResult;

    #[test]
    fn test_create_sitemap() {
        // Create an empty sitemap
        let mut sitemap = Sitemap::new();

        // Create a SiteMapData entry
        let entry = SiteMapData {
            loc: Url::parse("http://example.com")
                .expect("Failed to parse URL"),
            lastmod: "2024-10-08".to_string(),
            changefreq: ChangeFreq::Daily,
        };

        // Add the entry to the sitemap
        sitemap.add_entry(entry).expect("Failed to add entry");

        // Verify the sitemap contains the correct data
        assert_eq!(sitemap.len(), 1);
        assert!(!sitemap.is_empty());
    }

    #[test]
    fn test_serialize_sitemap() {
        // Create a new sitemap and add an entry
        let mut sitemap = Sitemap::new();
        let entry = SiteMapData {
            loc: Url::parse("http://example.com")
                .expect("Failed to parse URL"),
            lastmod: "2024-10-08".to_string(),
            changefreq: ChangeFreq::Daily,
        };

        sitemap.add_entry(entry).expect("Failed to add entry");

        // Serialize the sitemap to XML
        let serialized =
            sitemap.to_xml().expect("Failed to serialize sitemap");

        // Assert that the serialized XML contains the correct information
        assert!(serialized.contains("<url>"));
        assert!(serialized.contains("<loc>http://example.com/</loc>")); // Note the trailing slash
        assert!(serialized.contains("<changefreq>daily</changefreq>"));
        assert!(serialized.contains("<lastmod>2024-10-08</lastmod>"));
    }

    #[test]
    fn test_invalid_url_error() {
        // Try to add an entry with an invalid URL and expect an error
        let mut sitemap = Sitemap::new();

        let invalid_url = Url::parse("invalid-url");
        let result = match invalid_url {
            Ok(valid_url) => sitemap.add_entry(SiteMapData {
                loc: valid_url,
                lastmod: "2024-10-08".to_string(),
                changefreq: ChangeFreq::Daily,
            }),
            Err(e) => Err(SitemapError::UrlError(e)),
        };

        // Assert that the result is an error due to an invalid URL
        assert!(matches!(result, Err(SitemapError::UrlError(_))));
    }

    #[test]
    fn test_convert_date_format() {
        // Test converting date formats using the helper function
        let date = "2024-10-08T00:00:00Z";
        let converted = convert_date_format(date);
        assert_eq!(converted, "2024-10-08");
    }

    #[test]
    fn test_change_freq_enum() {
        // Test the ChangeFreq enum values
        assert_eq!(ChangeFreq::Daily.to_string(), "daily");
        assert_eq!(ChangeFreq::Monthly.to_string(), "monthly");
    }

    #[test]
    fn test_sitemap_data_creation() {
        // Test creating a new SiteMapData instance
        let sitemap_entry = SiteMapData {
            loc: Url::parse("http://example.com")
                .expect("Failed to parse URL"),
            lastmod: "2024-10-08".to_string(),
            changefreq: ChangeFreq::Daily,
        };

        // Create an empty sitemap and add the entry
        let mut sitemap = Sitemap::new();
        sitemap
            .add_entry(sitemap_entry)
            .expect("Failed to add entry");

        // Check that the entry was added
        assert_eq!(sitemap.len(), 1);
    }

    #[test]
    fn test_sitemap_error_handling() {
        // Test various error types defined in SitemapError
        let url_error: SitemapError =
            SitemapError::UrlError(url::ParseError::EmptyHost);
        let io_error: SitemapError =
            SitemapError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found",
            ));

        assert!(matches!(url_error, SitemapError::UrlError(_)));
        assert!(matches!(io_error, SitemapError::IoError(_)));
    }

    #[test]
    fn test_sitemap_result() {
        // Test that SitemapResult works with Ok and Err variants
        let success: SitemapResult<&str> = Ok("Success");
        let failure: SitemapResult<&str> =
            Err(SitemapError::UrlError(url::ParseError::EmptyHost));

        assert!(success.is_ok());
        assert!(failure.is_err());
    }
}
