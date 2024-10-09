#![allow(missing_docs)]
use sitemap_gen::utils::{
    create_cli, is_valid_url, normalize_urls,
    read_urls_from_file, write_output, format_date,
};
use dtt::dtt_now;
use sitemap_gen::error::SitemapError;

/// Entry point for the sitemap-gen utility examples.
///
/// This function runs various examples demonstrating how to use the utility
/// functions for sitemap generation and URL handling.
///
/// # Errors
///
/// Returns an error if any of the example functions fail.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 Sitemap Utility Examples\n");
    cli_example()?;
    read_urls_example()?;
    normalize_urls_example()?;
    write_output_example()?;
    format_date_example()?;
    valid_url_example()?;
    println!("\n🎉 All utility examples completed successfully!");
    Ok(())
}

/// Demonstrates creating and using a CLI for the sitemap generator.
fn cli_example() -> Result<(), SitemapError> {
    println!("🦀 CLI Creation Example");
    println!("---------------------------------------------");

    let cli = create_cli();
    cli.clone().print_help().map_err(SitemapError::IoError)?;

    println!("\n    ✅ CLI created and help printed successfully.");
    Ok(())
}

/// Demonstrates reading URLs from a file.
fn read_urls_example() -> Result<(), SitemapError> {
    println!("\n🦀 Read URLs from File Example");
    println!("---------------------------------------------");

    let file_path = "examples/urls.txt"; // Simulate a file path
    let urls = read_urls_from_file(file_path)?;
    println!("    ✅ URLs read from file successfully: {:?}", urls);
    Ok(())
}

/// Demonstrates normalizing a list of URLs.
fn normalize_urls_example() -> Result<(), SitemapError> {
    println!("\n🦀 Normalize URLs Example");
    println!("---------------------------------------------");

    let urls = vec![
        "http://example.com".parse()?,
        "http://example.com/page#fragment".parse()?,
        "https://example.org".parse()?,
    ];

    let normalized_urls = normalize_urls(urls);
    println!(
        "    ✅ URLs normalized successfully: {:?}",
        normalized_urls
    );
    Ok(())
}

/// Demonstrates writing output to a file.
fn write_output_example() -> Result<(), SitemapError> {
    println!("\n🦀 Write Output Example");
    println!("---------------------------------------------");

    let xml_content = "<sitemap>...</sitemap>"; // Simulated XML content
    let output_file = "sitemap.xml"; // Simulated output file
    write_output(xml_content, output_file)?;

    println!("    ✅ Sitemap XML written to file successfully.");
    Ok(())
}

/// Demonstrates formatting a date for sitemap use.
fn format_date_example() -> Result<(), SitemapError> {
    println!("\n🦀 Format Date Example");
    println!("---------------------------------------------");

    let now = dtt_now!();
    let formatted_date = format_date(now);

    println!("    ✅ Current date formatted successfully: {}", formatted_date);
    Ok(())
}

/// Demonstrates checking the validity of URLs.
fn valid_url_example() -> Result<(), SitemapError> {
    println!("\n🦀 Valid URL Check Example");
    println!("---------------------------------------------");

    let url = "http://example.com".parse()?;
    if is_valid_url(&url) {
        println!("    ✅ URL is valid: {}", url);
    } else {
        println!("    ❌ URL is invalid: {}", url);
    }

    Ok(())
}
