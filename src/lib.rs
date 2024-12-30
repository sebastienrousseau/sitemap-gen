// Copyright © 2025 Sitemap Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![doc = include_str!("../README.md")]
#![doc(
    html_favicon_url = "https://kura.pro/sitemap-gen/images/favicon.ico",
    html_logo_url = "https://kura.pro/sitemap-gen/images/logos/sitemap-gen.svg",
    html_root_url = "https://docs.rs/sitemap-gen"
)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    missing_docs
)]
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::result_unit_err,
    clippy::clone_on_ref_ptr
)]

//! # Sitemap Generator Library
//!
//! A comprehensive Rust library for generating and managing XML sitemaps according to the
//! [Sitemaps XML format](https://www.sitemaps.org/protocol.html) specification.
//!
//! ## Key Features
//!
//! - Create and manage XML sitemaps with proper validation
//! - Support for URL normalization and deduplication
//! - Customizable change frequencies and last modification dates
//! - Comprehensive error handling with detailed diagnostics
//! - Size and entry count validation according to sitemap standards
//!
//! ## Example Usage
//!
//! ```rust
//! use sitemap_gen::prelude::*;
//! use url::Url;
//!
//! # fn main() -> SitemapResult<()> {
//! let mut sitemap = Sitemap::new();
//!
//! // Create a sitemap entry
//! let entry = SiteMapData {
//!     loc: Url::parse("https://example.com")?,
//!     lastmod: "2024-10-08".to_string(),
//!     changefreq: ChangeFreq::Daily,
//! };
//!
//! // Add the entry and generate XML
//! sitemap.add_entry(entry)?;
//! let xml = sitemap.to_xml()?;
//! # Ok(())
//! # }
//! ```

/// Configuration constants for sitemap generation and validation.
pub mod config {
    /// Maximum allowed size of a sitemap in bytes (10MB).
    pub const MAX_SITEMAP_SIZE: usize = 10 * 1024 * 1024;

    /// Maximum number of URLs allowed in a single sitemap.
    pub const MAX_URLS: usize = 50_000;

    /// Default XML namespace for sitemaps.
    pub const SITEMAP_XMLNS: &str =
        "http://www.sitemaps.org/schemas/sitemap/0.9";
}

/// Error types and handling for sitemap operations.
pub mod error;

/// Core sitemap functionality and data structures.
pub mod sitemap;

/// Utility functions for sitemap generation and management.
pub mod utils;

// Re-exports for convenience
pub use config::{MAX_SITEMAP_SIZE, MAX_URLS, SITEMAP_XMLNS};
pub use error::SitemapError;
pub use sitemap::{
    convert_date_format, create_site_map_data, ChangeFreq, SiteMapData,
    Sitemap,
};

/// Current crate version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Result type alias for sitemap operations.
///
/// This type is used throughout the library to handle operations that might fail.
/// The error type is always [`SitemapError`].
///
/// See [`error::SitemapError`] for more details about possible error conditions.
pub type SitemapResult<T> = Result<T, SitemapError>;

/// Prelude module providing commonly used types and traits.
///
/// This module re-exports the most frequently used types and traits from the library,
/// allowing users to import them with a single `use` statement.
///
/// # Example
///
/// ```rust
/// use sitemap_gen::prelude::*;
///
/// # fn main() -> SitemapResult<()> {
/// let sitemap = Sitemap::new();
/// # Ok(())
/// # }
/// ```
pub mod prelude {
    pub use crate::config::{MAX_SITEMAP_SIZE, MAX_URLS};
    pub use crate::error::SitemapError;
    pub use crate::sitemap::{ChangeFreq, SiteMapData, Sitemap};
    pub use crate::SitemapResult;
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn test_sitemap_creation() -> SitemapResult<()> {
        let mut sitemap = Sitemap::new();
        let entry = SiteMapData {
            loc: Url::parse("http://example.com")?,
            lastmod: "2024-10-08".to_string(),
            changefreq: ChangeFreq::Daily,
        };

        sitemap.add_entry(entry)?;
        assert_eq!(sitemap.len(), 1);
        assert!(!sitemap.is_empty());
        Ok(())
    }

    #[test]
    fn test_sitemap_serialization() -> SitemapResult<()> {
        let mut sitemap = Sitemap::new();
        let entry = SiteMapData {
            loc: Url::parse("http://example.com")?,
            lastmod: "2024-10-08".to_string(),
            changefreq: ChangeFreq::Daily,
        };

        sitemap.add_entry(entry)?;
        let xml = sitemap.to_xml()?;

        assert!(xml.contains("<urlset"));
        assert!(xml.contains("<url>"));
        assert!(xml.contains("<loc>http://example.com/</loc>"));
        assert!(xml.contains("<changefreq>daily</changefreq>"));
        assert!(xml.contains("<lastmod>2024-10-08</lastmod>"));
        Ok(())
    }

    #[test]
    fn test_invalid_url() {
        let mut sitemap = Sitemap::new();
        let result = Url::parse("invalid-url").map(|url| {
            sitemap.add_entry(SiteMapData {
                loc: url,
                lastmod: "2024-10-08".to_string(),
                changefreq: ChangeFreq::Daily,
            })
        });

        assert!(result.is_err());
    }

    #[test]
    fn test_date_conversion() -> () {
        let formatted = convert_date_format("20 May 2023");
        assert_eq!(formatted, "2023-05-20");
    }

    #[test]
    fn test_size_limits() -> SitemapResult<()> {
        let mut sitemap = Sitemap::new();
        let url = Url::parse("http://example.com")?;

        // Add MAX_URLS entries
        for i in 0..MAX_URLS {
            sitemap.add_entry(SiteMapData {
                loc: Url::parse(&format!("{}?id={}", url, i))?,
                lastmod: "2024-10-08".to_string(),
                changefreq: ChangeFreq::Daily,
            })?;
        }

        // Try to add one more
        let result = sitemap.add_entry(SiteMapData {
            loc: url,
            lastmod: "2024-10-08".to_string(),
            changefreq: ChangeFreq::Daily,
        });

        assert!(result.is_err());
        Ok(())
    }
}
