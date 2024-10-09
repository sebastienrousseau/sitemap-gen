#![allow(missing_docs)]
use sitemap_gen::sitemap::{create_site_map_data, SiteMapData, Sitemap, ChangeFreq};
use sitemap_gen::error::SitemapError;
use url::Url;
use std::collections::HashMap;

/// Entry point for the sitemap-gen usage examples.
///
/// This function runs various examples demonstrating how to create, add, and convert sitemaps,
/// and how to handle errors encountered during the process.
///
/// # Errors
///
/// Returns an error if any of the example functions fail.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 sitemap-gen Usage Examples\n");
    create_site_map_data_example()?;
    add_entry_to_sitemap_example()?;
    convert_sitemap_to_xml_example()?;
    sitemap_size_limit_example()?;
    println!("\n🎉 All sitemap examples completed successfully!");
    Ok(())
}

/// Demonstrates creating `SiteMapData` from metadata.
fn create_site_map_data_example() -> Result<(), SitemapError> {
    println!("🦀 Create SiteMapData Example");
    println!("---------------------------------------------");

    let mut metadata = HashMap::new();
    let _ = metadata.insert("last_build_date".to_string(), "20 May 2023".to_string());
    let _ = metadata.insert("changefreq".to_string(), "weekly".to_string());
    let _ = metadata.insert("permalink".to_string(), "https://example.com".to_string());

    let site_map_data = create_site_map_data(&metadata)?;

    println!("    ✅ Created SiteMapData: {:?}", site_map_data);
    Ok(())
}

/// Demonstrates adding an entry to the sitemap.
fn add_entry_to_sitemap_example() -> Result<(), SitemapError> {
    println!("\n🦀 Add Entry to Sitemap Example");
    println!("---------------------------------------------");

    let mut sitemap = Sitemap::new();
    let entry = SiteMapData {
        loc: Url::parse("https://example.com")?,
        lastmod: "2023-05-20".to_string(),
        changefreq: ChangeFreq::Weekly,
    };

    sitemap.add_entry(entry)?;
    println!("    ✅ Successfully added entry to sitemap. Total entries: {}", sitemap.len());
    Ok(())
}

/// Demonstrates converting a sitemap to XML.
fn convert_sitemap_to_xml_example() -> Result<(), SitemapError> {
    println!("\n🦀 Convert Sitemap to XML Example");
    println!("---------------------------------------------");

    let mut sitemap = Sitemap::new();
    sitemap.add_entry(SiteMapData {
        loc: Url::parse("https://example.com")?,
        lastmod: "2023-05-20".to_string(),
        changefreq: ChangeFreq::Weekly,
    })?;

    let xml = sitemap.to_xml()?;
    println!("    ✅ Sitemap XML generated successfully:\n{}", xml);
    Ok(())
}

/// Demonstrates handling the sitemap size limit.
fn sitemap_size_limit_example() -> Result<(), SitemapError> {
    println!("\n🦀 Sitemap Size Limit Example");
    println!("---------------------------------------------");

    let mut sitemap = Sitemap::new();
    for i in 0..50_000 {
        sitemap.add_entry(SiteMapData {
            loc: Url::parse(&format!("https://example.com/{}", i))?,
            lastmod: "2023-05-20".to_string(),
            changefreq: ChangeFreq::Weekly,
        })?;
    }

    let result = sitemap.add_entry(SiteMapData {
        loc: Url::parse("https://example.com/toomany")?,
        lastmod: "2023-05-20".to_string(),
        changefreq: ChangeFreq::Weekly,
    });

    match result {
        Ok(_) => {
            println!("    ❌ Unexpected success in adding entry beyond size limit");
        }
        Err(SitemapError::MaxUrlLimitExceeded(limit)) => {
            println!("    ✅ Successfully caught sitemap size limit error at {}", limit);
        }
        Err(e) => {
            println!("    ❌ Unexpected error: {}", e);
        }
    }
    Ok(())
}
