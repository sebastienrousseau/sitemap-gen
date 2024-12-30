// Copyright © 2025 Sitemap Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Error types for the sitemap library.
//!
//! This module defines various error types that can occur during sitemap operations,
//! including XML parsing, date handling, URL parsing, and I/O operations.
//!
//! The main error type is `SitemapError`, which encapsulates all possible errors
//! that can occur within the library. This allows for consistent error handling
//! and propagation throughout the codebase.

use dtt::error::DateTimeError;
use std::string::FromUtf8Error;
use thiserror::Error;

/// Errors that can occur when working with sitemaps.
///
/// This enum represents all possible errors that can occur within the sitemap library.
/// It uses the `thiserror` crate for deriving the `Error` trait, which simplifies
/// error handling and provides good interoperability with the standard library.
///
/// The `non_exhaustive` attribute allows for future expansion of the error types
/// without breaking backwards compatibility.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum SitemapError {
    /// Error occurred during XML writing.
    #[error("XML writing error: {0}")]
    XmlWriteError(#[from] xml::writer::Error),

    /// Error occurred during XML parsing.
    #[error("XML parsing error: {0}")]
    XmlParseError(#[from] xml::reader::Error),

    /// Error occurred during date parsing or formatting.
    #[error("Date error: {0}")]
    DateError(#[from] DateTimeError),

    /// Error occurred during URL parsing.
    #[error("URL error: {0}")]
    UrlError(#[from] url::ParseError),

    /// Error occurred during I/O operations.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Error occurred during string encoding.
    #[error("Encoding error: {0}")]
    EncodingError(#[from] FromUtf8Error),

    /// Invalid change frequency provided.
    #[error("Invalid change frequency: {0}")]
    InvalidChangeFreq(String),

    /// Custom error for unforeseen scenarios.
    #[error("Custom error: {0}")]
    CustomError(String),

    /// Error occurred when a sitemap exceeds the maximum allowed size.
    #[error("Sitemap size exceeds the maximum allowed (10MB)")]
    SitemapTooLarge,

    /// Error occurred when the number of URLs in a sitemap exceeds the maximum allowed.
    #[error("Number of URLs ({0}) exceeds the maximum allowed limit (50,000)")]
    MaxUrlLimitExceeded(usize),
}

impl SitemapError {
    /// Provides additional context for the error.
    ///
    /// This method returns a static string that gives more information about
    /// the context in which the error occurred. This can be useful for logging
    /// or providing more detailed error messages to users.
    ///
    /// # Returns
    /// A string slice describing the context of the error.
    #[must_use]
    pub const fn context(&self) -> &'static str {
        match self {
            Self::XmlWriteError(_) => "Error occurred while writing XML data",
            Self::XmlParseError(_) => "Error occurred while parsing XML data",
            Self::DateError(_) => "Error occurred while parsing or formatting dates",
            Self::UrlError(_) => "Error occurred while parsing URLs",
            Self::IoError(_) => "Error occurred during file or network operations",
            Self::EncodingError(_) => "Error occurred during UTF-8 string encoding or decoding",
            Self::InvalidChangeFreq(_) => "An invalid change frequency value was provided",
            Self::CustomError(_) => "An unexpected error occurred",
            Self::SitemapTooLarge => "The generated sitemap exceeds the maximum allowed size",
            Self::MaxUrlLimitExceeded(_) => "The number of URLs exceeds the maximum allowed limit",
        }
    }
}

/// Custom result type for sitemap operations.
///
/// This type alias simplifies the return types of functions that can produce
/// a `SitemapError`. It's a convenient shorthand for `Result<T, SitemapError>`.
pub type SitemapResult<T> = Result<T, SitemapError>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use xml::writer::{EventWriter, XmlEvent};

    #[test]
    fn test_error_creation_and_formatting() {
        // Create a XML writing error
        let mut writer = EventWriter::new(Vec::new());
        let xml_write_result = writer.write(XmlEvent::end_element()); // This will cause an error because we're ending an element that wasn't started
        let xml_write_error = xml_write_result.unwrap_err();
        let sitemap_error =
            SitemapError::XmlWriteError(xml_write_error);
        assert!(sitemap_error
            .to_string()
            .contains("XML writing error"));
        assert_eq!(
            sitemap_error.context(),
            "Error occurred while writing XML data"
        );

        let custom_error =
            SitemapError::CustomError("Test error".to_string());
        assert_eq!(
            custom_error.to_string(),
            "Custom error: Test error"
        );
    }

    #[test]
    fn test_error_context() {
        let url_error =
            SitemapError::UrlError(url::ParseError::EmptyHost);
        assert_eq!(
            url_error.context(),
            "Error occurred while parsing URLs"
        );

        let io_error = SitemapError::IoError(io::Error::new(
            io::ErrorKind::Other,
            "I/O Error",
        ));
        assert_eq!(
            io_error.context(),
            "Error occurred during file or network operations"
        );

        let invalid_change_freq =
            SitemapError::InvalidChangeFreq("invalid".to_string());
        assert_eq!(
            invalid_change_freq.context(),
            "An invalid change frequency value was provided"
        );
    }

    #[test]
    fn test_error_display() {
        let date_error =
            SitemapError::DateError(DateTimeError::InvalidFormat);
        assert_eq!(
            date_error.to_string(),
            "Date error: Invalid date format"
        );

        let url_error =
            SitemapError::UrlError(url::ParseError::EmptyHost);
        assert_eq!(url_error.to_string(), "URL error: empty host");

        let io_error = SitemapError::IoError(io::Error::new(
            io::ErrorKind::Other,
            "I/O Error",
        ));
        assert_eq!(io_error.to_string(), "I/O error: I/O Error");

        let custom_error = SitemapError::CustomError(
            "Custom error message".to_string(),
        );
        assert_eq!(
            custom_error.to_string(),
            "Custom error: Custom error message"
        );

        let sitemap_too_large = SitemapError::SitemapTooLarge;
        assert_eq!(
            sitemap_too_large.to_string(),
            "Sitemap size exceeds the maximum allowed (10MB)"
        );

        let max_url_limit_exceeded =
            SitemapError::MaxUrlLimitExceeded(60000);
        assert_eq!(
            max_url_limit_exceeded.to_string(),
            "Number of URLs (60000) exceeds the maximum allowed limit (50,000)"
        );
    }

    #[test]
    fn test_result_type_alias() {
        fn demo_function() -> SitemapResult<()> {
            Err(SitemapError::CustomError("Test error".to_string()))
        }

        let result = demo_function();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            SitemapError::CustomError(_)
        ));
    }

    #[test]
    fn test_xml_parse_error() {
        let xml = "<invalid>";
        let reader = xml::reader::EventReader::from_str(xml);
        let parse_result: Result<
            Vec<xml::reader::XmlEvent>,
            xml::reader::Error,
        > = reader.into_iter().collect();
        let xml_parse_error = parse_result.unwrap_err();
        let sitemap_error =
            SitemapError::XmlParseError(xml_parse_error);
        assert!(sitemap_error
            .to_string()
            .contains("XML parsing error"));
        assert_eq!(
            sitemap_error.context(),
            "Error occurred while parsing XML data"
        );
    }

    #[test]
    fn test_date_error() {
        let date_error =
            SitemapError::DateError(DateTimeError::InvalidFormat);
        assert_eq!(
            date_error.context(),
            "Error occurred while parsing or formatting dates"
        );
    }

    #[test]
    fn test_encoding_error() {
        let invalid_utf8 = vec![0xFF, 0xFE, 0xFD];
        let encoding_error =
            String::from_utf8(invalid_utf8).unwrap_err();
        let sitemap_error = SitemapError::EncodingError(encoding_error);
        assert!(sitemap_error.to_string().contains("Encoding error"));
        assert_eq!(
            sitemap_error.context(),
            "Error occurred during UTF-8 string encoding or decoding"
        );
    }

    #[test]
    fn test_sitemap_size_errors() {
        let sitemap_too_large = SitemapError::SitemapTooLarge;
        assert_eq!(
            sitemap_too_large.to_string(),
            "Sitemap size exceeds the maximum allowed (10MB)"
        );
        assert_eq!(
            sitemap_too_large.context(),
            "The generated sitemap exceeds the maximum allowed size"
        );

        let max_url_limit_exceeded =
            SitemapError::MaxUrlLimitExceeded(60000);
        assert_eq!(
            max_url_limit_exceeded.to_string(),
            "Number of URLs (60000) exceeds the maximum allowed limit (50,000)"
        );
        assert_eq!(
            max_url_limit_exceeded.context(),
            "The number of URLs exceeds the maximum allowed limit"
        );
    }

    #[test]
    fn test_error_propagation() {
        fn parse_url() -> SitemapResult<()> {
            Err(SitemapError::UrlError(url::ParseError::EmptyHost))
        }

        fn handle_url() -> SitemapResult<()> {
            parse_url()?;
            Ok(())
        }

        let result = handle_url();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            SitemapError::UrlError(_)
        ));
    }

    #[test]
    fn test_url_parsing_errors() {
        let empty_host =
            SitemapError::UrlError(url::ParseError::EmptyHost);
        assert_eq!(empty_host.to_string(), "URL error: empty host");

        let invalid_port =
            SitemapError::UrlError(url::ParseError::InvalidPort);
        assert_eq!(
            invalid_port.to_string(),
            "URL error: invalid port number"
        ); // Adjusted expected message

        let relative_url = SitemapError::UrlError(
            url::ParseError::RelativeUrlWithoutBase,
        );
        assert_eq!(
            relative_url.to_string(),
            "URL error: relative URL without a base"
        );
    }

    #[test]
    fn test_invalid_change_freq_edge_cases() {
        let empty_string =
            SitemapError::InvalidChangeFreq("".to_string());
        assert_eq!(
            empty_string.to_string(),
            "Invalid change frequency: "
        );

        let long_string =
            SitemapError::InvalidChangeFreq("a".repeat(1000));
        assert!(long_string
            .to_string()
            .contains("Invalid change frequency"));
    }

    #[test]
    fn test_max_url_limit_exceeded_edge_cases() {
        let just_under_limit = SitemapError::MaxUrlLimitExceeded(49999);
        assert_eq!(just_under_limit.to_string(), "Number of URLs (49999) exceeds the maximum allowed limit (50,000)");

        let at_limit = SitemapError::MaxUrlLimitExceeded(50000);
        assert_eq!(at_limit.to_string(), "Number of URLs (50000) exceeds the maximum allowed limit (50,000)");

        let over_limit = SitemapError::MaxUrlLimitExceeded(50001);
        assert_eq!(over_limit.to_string(), "Number of URLs (50001) exceeds the maximum allowed limit (50,000)");
    }
}
