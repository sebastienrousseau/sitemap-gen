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
