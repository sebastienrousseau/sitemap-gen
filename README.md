<p align="center">
  <img src="https://kura.pro/sitemap-gen/images/logos/sitemap-gen.svg" alt="Sitemap Gen logo" width="128" />
</p>

<h1 align="center">Sitemap Gen</h1>

<p align="center">
  <strong>A Rust library for efficient generation and optimization of sitemaps.</strong>
</p>

<p align="center">
  <a href="https://github.com/sebastienrousseau/sitemap-gen/actions"><img src="https://img.shields.io/github/actions/workflow/status/sebastienrousseau/sitemap-gen/ci.yml?style=for-the-badge&logo=github" alt="Build" /></a>
  <a href="https://crates.io/crates/sitemap-gen"><img src="https://img.shields.io/crates/v/sitemap-gen.svg?style=for-the-badge&color=fc8d62&logo=rust" alt="Crates.io" /></a>
  <a href="https://docs.rs/sitemap-gen"><img src="https://img.shields.io/badge/docs.rs-sitemap-gen-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" alt="Docs.rs" /></a>
  <a href="https://codecov.io/gh/sebastienrousseau/sitemap-gen"><img src="https://img.shields.io/codecov/c/github/sebastienrousseau/sitemap-gen?style=for-the-badge&logo=codecov" alt="Coverage" /></a>
  <a href="https://lib.rs/crates/sitemap-gen"><img src="https://img.shields.io/badge/lib.rs-v0.0.2-orange.svg?style=for-the-badge" alt="lib.rs" /></a>
</p>

---

## Install

```bash
cargo add sitemap-gen
```

Or add to `Cargo.toml`:

```toml
[dependencies]
sitemap-gen = "0.0.5"
```

You need [Rust](https://rustup.rs/) 1.56.0 or later. Works on macOS, Linux, and Windows.

---

## Overview

Sitemap Gen generates and optimises XML sitemaps for search engine indexing.

- **Standard sitemaps** from URL lists
- **Sitemap index** for large sites
- **News, image, and video** sitemap extensions
- **Compression** for optimised delivery

---

## Features

| | |
| :--- | :--- |
| **Sitemap generation** | Generate XML sitemaps from URL lists |
| **Multiple formats** | Standard sitemap, sitemap index, news, image, and video |
| **Optimisation** | Compress and optimise sitemaps for search engines |
| **Validation** | Validate URLs and sitemap structure |
| **CLI** | Command-line interface for batch generation |

---

## Usage

```rust
use sitemap_gen::{ChangeFreq, SiteMapData, Sitemap};
use url::Url;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sitemap = Sitemap::new();

    sitemap.add_entry(SiteMapData {
        loc: Url::parse("https://example.com/")?,
        lastmod: "2026-06-21".to_string(),
        changefreq: ChangeFreq::Weekly,
    })?;

    let xml = sitemap.to_xml()?;
    println!("{}", xml);
    Ok(())
}
```

---

## Development

```bash
cargo build        # Build the project
cargo test         # Run all tests
cargo clippy       # Lint with Clippy
cargo fmt          # Format with rustfmt
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup, signed commits, and PR guidelines.

---

**THE ARCHITECT** ᴫ [Sebastien Rousseau](https://sebastienrousseau.com)
**THE ENGINE** ᵞ [EUXIS](https://euxis.co) ᴫ Enterprise Unified Execution Intelligence System

---

## License

Dual-licensed under [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0) or [MIT](https://opensource.org/licenses/MIT), at your option.

<p align="right"><a href="#sitemap-gen">Back to Top</a></p>
