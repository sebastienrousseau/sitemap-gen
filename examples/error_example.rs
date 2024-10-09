#![allow(missing_docs)]
use dtt::datetime::DateTime;
use sitemap_gen::error::SitemapError;

/// Entry point for the sitemap-gen error handling examples.
///
/// This function runs various examples demonstrating error creation, conversion,
/// and handling for different scenarios in the sitemap-gen library.
///
/// # Errors
///
/// Returns an error if any of the example functions fail.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 sitemap-gen Error Handling Examples\n");
    url_error_example()?;
    date_error_example()?;
    xml_error_example()?;
    io_error_example()?;
    encoding_error_example()?;
    invalid_change_freq_error_example()?;
    println!(
        "\n🎉 All error handling examples completed successfully!"
    );
    Ok(())
}

/// Demonstrates handling of URL parsing errors.
fn url_error_example() -> Result<(), SitemapError> {
    println!("🦀 URL Parsing Error Example");
    println!("---------------------------------------------");
    let invalid_url = "htp:/example.com";
    let result = invalid_url.parse::<url::Url>();
    match result {
        Ok(_) => {
            println!("    ❌ Unexpected success in parsing URL")
        }
        Err(e) => {
            let error = SitemapError::UrlError(e);
            println!("    ✅ Successfully caught URL Error: {}", error);
        }
    }
    Ok(())
}

/// Demonstrates handling of date formatting or parsing errors.
fn date_error_example() -> Result<(), SitemapError> {
    println!("\n🦀 Date Parsing Error Example");
    println!("---------------------------------------------");
    let result = DateTime::parse("invalid_date");
    match result {
        Ok(_) => {
            println!("    ❌ Unexpected success in parsing date")
        }
        Err(e) => {
            let error = SitemapError::DateError(e);
            println!(
                "    ✅ Successfully caught Date Error: {}",
                error
            );
        }
    }
    Ok(())
}

/// Demonstrates handling of XML errors.
fn xml_error_example() -> Result<(), SitemapError> {
    println!("\n🦀 XML Error Example");
    println!("---------------------------------------------");
    // Create a situation where writing XML might fail
    let mut writer =
        xml::writer::EmitterConfig::new().create_writer(Vec::new());
    // Write a start element
    let result =
        writer.write(xml::writer::XmlEvent::start_element("root"));
    // Now write the corresponding end element
    let result = result.and_then(|_| {
        writer.write(xml::writer::XmlEvent::end_element())
    });
    match result {
        Ok(_) => {
            println!("    ❌ Unexpected success in writing XML")
        }
        Err(e) => {
            let error = SitemapError::XmlWriteError(e);
            println!("    ✅ Successfully caught XML Error: {}", error);
        }
    }
    Ok(())
}

/// Demonstrates handling of IO errors.
fn io_error_example() -> Result<(), SitemapError> {
    println!("\n🦀 IO Error Example");
    println!("---------------------------------------------");
    let io_error = std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "File not found",
    );
    let error = SitemapError::IoError(io_error);
    println!("    ✅ Created IO Error: {}", error);
    Ok(())
}

/// Demonstrates handling of string encoding errors.
fn encoding_error_example() -> Result<(), SitemapError> {
    println!("\n🦀 Encoding Error Example");
    println!("---------------------------------------------");
    let invalid_utf8 = vec![0xFF, 0xFE, 0xFD]; // Invalid UTF-8 sequence
    let encoding_error = String::from_utf8(invalid_utf8).unwrap_err();
    let error = SitemapError::EncodingError(encoding_error);
    println!("    ✅ Created Encoding Error: {}", error);
    Ok(())
}

/// Demonstrates handling of invalid change frequency errors.
fn invalid_change_freq_error_example() -> Result<(), SitemapError> {
    println!("\n🦀 Invalid Change Frequency Error Example");
    println!("---------------------------------------------");
    let error = SitemapError::InvalidChangeFreq("monthly".to_string());
    println!(
        "    ✅ Created Invalid Change Frequency Error: {}",
        error
    );
    Ok(())
}
