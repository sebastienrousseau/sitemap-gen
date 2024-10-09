//! Examples demonstrating the usage of the `sitemap-gen` library.
//!
//! These examples cover basic operations like sitemap creation, error handling,
//! and more advanced topics like custom date formats and change frequencies.

use sitemap_gen::convert_date_format;
use sitemap_gen::prelude::*;
use url::Url;

/// Main entry point for the sitemap-gen examples.
///
/// This function demonstrates various ways to use the sitemap-gen library,
/// including creating sitemaps, adding entries, and error handling.
///
/// # Errors
///
/// Returns an error if any of the sitemap creation or modification fails.
fn main() -> SitemapResult<()> {
    println!("\n🧪 sitemap-gen Usage Examples\n");

    // Example: Create a new sitemap and add a URL
    create_sitemap_example()?;

    // Example: Demonstrate error handling for invalid data
    handle_invalid_date_error()?;
    handle_invalid_url_error()?;

    println!("\n🎉 All sitemap-gen examples completed successfully!");
    Ok(())
}

/// Example demonstrating how to create a sitemap and add a URL.
fn create_sitemap_example() -> SitemapResult<()> {
    println!("🦀 Creating Sitemap Example");
    println!("---------------------------------------------");

    // Create a new sitemap
    let mut sitemap = Sitemap::new();

    // Define a valid URL and create a SiteMapData entry
    let url = Url::parse("https://example.com/")?;
    let site_data = SiteMapData {
        loc: url,
        lastmod: "2023-10-09".to_string(),
        changefreq: ChangeFreq::Daily,
    };

    // Add the site data to the sitemap
    sitemap.add_entry(site_data)?;

    // Convert the sitemap to XML and display it
    let xml_output = sitemap.to_xml()?; // `to_xml` returns a `String`
    println!("✅ Sitemap generated:\n{}", xml_output);

    Ok(())
}

/// Example demonstrating handling of an invalid date error.
fn handle_invalid_date_error() -> SitemapResult<()> {
    println!("\n🦀 Handling Invalid Date Error");
    println!("---------------------------------------------");

    // Try to create a SiteMapData with an invalid date
    let invalid_date = "not-a-date";
    let result = convert_date_format(invalid_date); // Adjusted: Proper handling of Result

    if result == invalid_date {
        println!("    ✅ Successfully caught Invalid Date: {}", result);
    } else {
        println!("    ❌ Unexpected success in date conversion");
    }

    Ok(())
}

/// Example demonstrating handling of an invalid URL error.
fn handle_invalid_url_error() -> SitemapResult<()> {
    println!("\n🦀 Handling Invalid URL Error");
    println!("---------------------------------------------");

    let invalid_url = "htp:/example.com";
    let result = Url::parse(invalid_url);

    match result {
        Ok(_) => {
            println!("    ❌ Unexpected success in parsing URL");
        }
        Err(e) => {
            println!("    ✅ Successfully caught URL Error: {}", e);
        }
    }

    Ok(())
}
