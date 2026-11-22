// src/index.rs

//! Sitemap index files and splitting for sites with more than 50,000 URLs.
//!
//! The sitemaps.org protocol caps one sitemap at 50,000 URLs and 50 MB
//! uncompressed, and points crawlers at many sitemaps through one
//! `<sitemapindex>` file. [`split_entries`] cuts a list of entries into
//! sitemaps under the cap, [`SitemapIndex`] lists them, and
//! [`build_sitemap_set`] does both, naming each file from a base URL.

use crate::error::{SitemapError, SitemapResult};
use crate::sitemap::{SiteMapData, Sitemap};
use url::Url;

/// Maximum number of URLs in one sitemap, and of sitemaps in one index.
pub const MAX_ENTRIES_PER_FILE: usize = 50_000;

const SITEMAP_NS: &str = "http://www.sitemaps.org/schemas/sitemap/0.9";

/// One `<sitemap>` entry of a sitemap index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SitemapIndexEntry {
    /// The URL of the child sitemap.
    pub loc: Url,
    /// When the child sitemap last changed, in `YYYY-MM-DD` form.
    pub lastmod: Option<String>,
}

/// A sitemap index: the file that lists every sitemap of a large site.
#[derive(Debug, Default, Clone)]
pub struct SitemapIndex {
    entries: Vec<SitemapIndexEntry>,
}

impl SitemapIndex {
    /// Creates an empty index.
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of sitemaps the index lists.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the index lists no sitemap.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The sitemaps the index lists, in the order they were added.
    pub fn entries(&self) -> &[SitemapIndexEntry] {
        &self.entries
    }

    /// Adds a sitemap, or updates the `lastmod` of one already listed
    /// with the same `loc`, so an index can be refreshed in place when
    /// only some of its sitemaps changed.
    ///
    /// # Errors
    ///
    /// [`SitemapError::MaxUrlLimitExceeded`] when a new sitemap would
    /// take the index past 50,000 entries.
    pub fn upsert(
        &mut self,
        entry: SitemapIndexEntry,
    ) -> SitemapResult<()> {
        if let Some(existing) =
            self.entries.iter_mut().find(|e| e.loc == entry.loc)
        {
            existing.lastmod = entry.lastmod;
            return Ok(());
        }
        if self.entries.len() >= MAX_ENTRIES_PER_FILE {
            return Err(SitemapError::MaxUrlLimitExceeded(
                self.entries.len(),
            ));
        }
        self.entries.push(entry);
        Ok(())
    }

    /// Serialises the index as a `<sitemapindex>` document.
    ///
    /// # Errors
    ///
    /// Returns an error if the XML tree cannot be built.
    pub fn to_xml(&self) -> SitemapResult<String> {
        let mut doc = oxml::Document::empty();
        let root = doc.append_element(
            doc.root(),
            Some(SITEMAP_NS),
            "sitemapindex",
        )?;
        let _ = doc.declare_namespace(root, None, SITEMAP_NS)?;
        for entry in &self.entries {
            let sitemap =
                doc.append_element(root, Some(SITEMAP_NS), "sitemap")?;
            let loc =
                doc.append_element(sitemap, Some(SITEMAP_NS), "loc")?;
            let _ = doc.append_text(loc, entry.loc.as_ref())?;
            if let Some(lastmod) = &entry.lastmod {
                let node = doc.append_element(
                    sitemap,
                    Some(SITEMAP_NS),
                    "lastmod",
                )?;
                let _ = doc.append_text(node, lastmod)?;
            }
        }
        let mut xml =
            String::with_capacity(self.entries.len() * 120 + 96);
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let options = oxml::SerialiseOptions {
            indent: None,
            empty_elements: oxml::EmptyElement::Expanded,
            newline: oxml::Newline::Lf,
        };
        let _ = doc.write_xml_with(&mut xml, options);
        Ok(xml)
    }
}

/// Splits entries into sitemaps of at most `max_per_file` URLs each,
/// keeping their order.
///
/// # Errors
///
/// [`SitemapError::CustomError`] when `max_per_file` is 0 or above
/// 50,000.
pub fn split_entries(
    entries: Vec<SiteMapData>,
    max_per_file: usize,
) -> SitemapResult<Vec<Sitemap>> {
    if max_per_file == 0 || max_per_file > MAX_ENTRIES_PER_FILE {
        return Err(SitemapError::CustomError(format!(
            "max_per_file must be between 1 and {MAX_ENTRIES_PER_FILE}, got {max_per_file}"
        )));
    }
    let mut sitemaps =
        Vec::with_capacity(entries.len().div_ceil(max_per_file));
    let mut current = Sitemap::new();
    for entry in entries {
        if current.len() == max_per_file {
            sitemaps.push(std::mem::take(&mut current));
        }
        current.add_entry(entry)?;
    }
    if !current.is_empty() {
        sitemaps.push(current);
    }
    Ok(sitemaps)
}

/// The files of a split sitemap: each child sitemap and the index.
#[derive(Debug, Clone)]
pub struct SitemapSet {
    /// Each child as `(file name, XML)`, named `sitemap-1.xml`,
    /// `sitemap-2.xml`, and so on.
    pub sitemaps: Vec<(String, String)>,
    /// The index listing every child under `base_url`.
    pub index: SitemapIndex,
}

/// Splits entries into sitemaps and builds the index that lists them.
///
/// Each child is named `sitemap-<n>.xml` and listed in the index as
/// `base_url` joined with that name, with the newest `lastmod` of its
/// entries.
///
/// # Errors
///
/// Returns an error if `max_per_file` is out of range, a child name
/// cannot be joined to `base_url`, or the XML cannot be built.
pub fn build_sitemap_set(
    entries: Vec<SiteMapData>,
    base_url: &Url,
    max_per_file: usize,
) -> SitemapResult<SitemapSet> {
    let mut set = SitemapSet {
        sitemaps: Vec::new(),
        index: SitemapIndex::new(),
    };
    for (i, sitemap) in
        split_entries(entries, max_per_file)?.iter().enumerate()
    {
        let name = format!("sitemap-{}.xml", i + 1);
        set.index.upsert(SitemapIndexEntry {
            loc: base_url.join(&name)?,
            lastmod: sitemap.latest_lastmod(),
        })?;
        set.sitemaps.push((name, sitemap.to_xml()?));
    }
    Ok(set)
}

/// Compresses a sitemap or index for serving as `.xml.gz`.
///
/// # Errors
///
/// Returns an I/O error if compression fails.
#[cfg(feature = "gzip")]
pub fn gzip(xml: &str) -> SitemapResult<Vec<u8>> {
    use flate2::{write::GzEncoder, Compression};
    use std::io::Write;

    let mut encoder =
        GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(xml.as_bytes())?;
    Ok(encoder.finish()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sitemap::ChangeFreq;

    fn entry(i: usize, lastmod: &str) -> SiteMapData {
        SiteMapData {
            loc: Url::parse(&format!("https://example.com/{i}"))
                .unwrap(),
            lastmod: lastmod.to_string(),
            changefreq: ChangeFreq::Weekly,
        }
    }

    #[test]
    fn split_keeps_order_and_caps_each_file() {
        let entries: Vec<_> =
            (0..7).map(|i| entry(i, "2026-01-01")).collect();
        let parts = split_entries(entries, 3).unwrap();
        let sizes: Vec<_> = parts.iter().map(Sitemap::len).collect();
        assert_eq!(sizes, [3, 3, 1]);
        assert!(parts[2]
            .to_xml()
            .unwrap()
            .contains("https://example.com/6"));
    }

    #[test]
    fn split_past_fifty_thousand_urls() {
        let entries: Vec<_> =
            (0..50_001).map(|i| entry(i, "2026-01-01")).collect();
        let parts =
            split_entries(entries, MAX_ENTRIES_PER_FILE).unwrap();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[1].len(), 1);
    }

    #[test]
    fn split_of_nothing_is_empty() {
        assert!(split_entries(Vec::new(), 10).unwrap().is_empty());
    }

    #[test]
    fn split_rejects_out_of_range_sizes() {
        assert!(split_entries(vec![entry(0, "")], 0).is_err());
        assert!(split_entries(vec![entry(0, "")], 50_001).is_err());
    }

    #[test]
    fn index_xml_lists_each_sitemap() {
        let mut index = SitemapIndex::new();
        index
            .upsert(SitemapIndexEntry {
                loc: Url::parse("https://example.com/sitemap-1.xml")
                    .unwrap(),
                lastmod: Some("2026-02-03".into()),
            })
            .unwrap();
        index
            .upsert(SitemapIndexEntry {
                loc: Url::parse("https://example.com/sitemap-2.xml")
                    .unwrap(),
                lastmod: None,
            })
            .unwrap();
        let xml = index.to_xml().unwrap();
        assert_eq!(
            xml,
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
             <sitemapindex xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\
             <sitemap><loc>https://example.com/sitemap-1.xml</loc>\
             <lastmod>2026-02-03</lastmod></sitemap>\
             <sitemap><loc>https://example.com/sitemap-2.xml</loc></sitemap>\
             </sitemapindex>"
        );
    }

    #[test]
    fn index_upsert_updates_lastmod_in_place() {
        let loc =
            Url::parse("https://example.com/sitemap-1.xml").unwrap();
        let mut index = SitemapIndex::new();
        for lastmod in ["2026-01-01", "2026-03-04"] {
            index
                .upsert(SitemapIndexEntry {
                    loc: loc.clone(),
                    lastmod: Some(lastmod.into()),
                })
                .unwrap();
        }
        assert_eq!(index.len(), 1);
        assert_eq!(
            index.entries()[0].lastmod.as_deref(),
            Some("2026-03-04")
        );
    }

    #[test]
    fn index_rejects_more_than_fifty_thousand_sitemaps() {
        let mut index = SitemapIndex::new();
        for i in 0..MAX_ENTRIES_PER_FILE {
            index
                .upsert(SitemapIndexEntry {
                    loc: Url::parse(&format!("https://e.com/{i}.xml"))
                        .unwrap(),
                    lastmod: None,
                })
                .unwrap();
        }
        let extra = SitemapIndexEntry {
            loc: Url::parse("https://e.com/extra.xml").unwrap(),
            lastmod: None,
        };
        assert!(matches!(
            index.upsert(extra),
            Err(SitemapError::MaxUrlLimitExceeded(_))
        ));
    }

    #[test]
    fn set_names_children_and_dates_them() {
        let entries = vec![
            entry(0, "2026-01-01"),
            entry(1, "2026-05-06"),
            entry(2, "2026-02-02"),
        ];
        let base = Url::parse("https://example.com/maps/").unwrap();
        let set = build_sitemap_set(entries, &base, 2).unwrap();
        let names: Vec<_> =
            set.sitemaps.iter().map(|(n, _)| n.as_str()).collect();
        assert_eq!(names, ["sitemap-1.xml", "sitemap-2.xml"]);
        let index = set.index.entries();
        assert_eq!(
            index[0].loc.as_str(),
            "https://example.com/maps/sitemap-1.xml"
        );
        assert_eq!(index[0].lastmod.as_deref(), Some("2026-05-06"));
        assert_eq!(index[1].lastmod.as_deref(), Some("2026-02-02"));
    }

    #[cfg(feature = "gzip")]
    #[test]
    fn gzip_round_trips() {
        use std::io::Read;
        let xml = "<?xml version=\"1.0\"?><urlset/>";
        let bytes = gzip(xml).unwrap();
        assert_eq!(&bytes[..2], [0x1f, 0x8b]);
        let mut out = String::new();
        let _ = flate2::read::GzDecoder::new(&bytes[..])
            .read_to_string(&mut out)
            .unwrap();
        assert_eq!(out, xml);
    }
}
