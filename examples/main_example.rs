#![allow(missing_docs)]
use sitemap_gen::error::SitemapError;
use sitemap_gen::utils::normalize_urls;

/// Entry point for the sitemap-gen CLI examples.
///
/// This function demonstrates the core functionality of the sitemap generator
/// and runs through multiple examples such as generating sitemaps, reading from
/// files, and handling common errors.
///
/// # Errors
///
/// Returns an error if any of the example functions fail.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 Sitemap Generator CLI Examples\n");

    generate_sitemap_example()?;
    read_urls_from_file_example()?;
    normalize_urls_example()?;
    progress_bar_example()?;

    println!("\n🎉 All sitemap-gen examples completed successfully!");

    Ok(())
}

/// Example for generating a basic sitemap.
fn generate_sitemap_example() -> Result<(), SitemapError> {
    println!("🦀 Sitemap Generation Example");
    println!("---------------------------------------------");

    let urls = vec![
        "http://example.com".parse().unwrap(),
        "http://example.com/about".parse().unwrap(),
    ];

    let mut sitemap = sitemap_gen::Sitemap::new();
    for url in urls {
        sitemap.add_entry(sitemap_gen::SiteMapData {
            loc: url,
            lastmod: "2024-10-09".to_string(),
            changefreq: sitemap_gen::ChangeFreq::Weekly,
        })?;
    }

    let xml = sitemap.to_xml()?;
    println!("    ✅ Generated sitemap XML:\n{}", xml);

    Ok(())
}

/// Example for reading URLs from a file and handling potential errors.
fn read_urls_from_file_example() -> Result<(), SitemapError> {
    println!("\n🦀 Read URLs from File Example");
    println!("---------------------------------------------");

    // This would usually read from an actual file, but here we simulate it
    let urls = vec![
        "http://example.com".parse::<url::Url>(),
        "http://invalid-url".parse::<url::Url>(),
    ];

    for (i, url) in urls.into_iter().enumerate() {
        match url {
            Ok(valid_url) => {
                println!("    ✅ Valid URL {}: {:?}", i + 1, valid_url)
            }
            Err(e) => println!("    ❌ Invalid URL {}: {}", i + 1, e),
        }
    }

    Ok(())
}

/// Example for normalizing URLs and removing duplicates.
fn normalize_urls_example() -> Result<(), SitemapError> {
    println!("\n🦀 Normalize URLs Example");
    println!("---------------------------------------------");

    let urls = vec![
        "http://example.com".parse().unwrap(),
        "http://example.com/".parse().unwrap(),
        "http://example.com/about".parse().unwrap(),
        "http://example.com/about#section".parse().unwrap(),
    ];

    let normalized = normalize_urls(urls);

    println!("    ✅ Normalized URLs:");
    for url in normalized {
        println!("      - {}", url);
    }

    Ok(())
}

/// Example for demonstrating a progress bar for large input files.
fn progress_bar_example() -> Result<(), SitemapError> {
    println!("\n🦀 Progress Bar Example");
    println!("---------------------------------------------");

    let urls: Vec<url::Url> = vec![
        "http://example.com".parse().unwrap(),
        "http://example.com/about".parse().unwrap(),
        "http://example.com/contact".parse().unwrap(),
    ];

    let progress_bar = indicatif::ProgressBar::new(urls.len() as u64);
    progress_bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("██-"),
    );

    for url in urls {
        progress_bar.set_message(format!("Processing: {}", url));
        progress_bar.inc(1);
        // Simulate processing delay
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    progress_bar.finish_with_message("All URLs processed!");

    Ok(())
}
