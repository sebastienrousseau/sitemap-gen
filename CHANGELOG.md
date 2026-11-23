# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.8] - 2026-11-22

### Added

- Sitemap index support for sites with more than 50,000 URLs (#12).
  `split_entries` cuts entries into sitemaps of at most 50,000 URLs,
  in order; `SitemapIndex` serialises a `<sitemapindex>` and refreshes
  one listed sitemap's `lastmod` in place through `upsert`;
  `build_sitemap_set` does both, naming the children `sitemap-<n>.xml`
  under a base URL and dating each from its newest entry.
- Incremental updates: `Sitemap::upsert_entry` replaces the entry with
  the same `loc` (or adds it), `Sitemap::remove_entry` drops one, and
  `Sitemap::latest_lastmod` returns the newest `lastmod`.
- `gzip` feature: `index::gzip` compresses a sitemap or index for
  serving as `.xml.gz`, with `flate2`'s pure-Rust backend. Off by
  default, so default builds gain no dependency.

### Security

- `url` 2.5.9 (RUSTSEC-2026-0592, stack overflow in `Url::origin()`
  on nested `blob:` URLs) and `rustix` 1.1.6 (RUSTSEC-2026-0486,
  xattr functions on a zero-length buffer), both transitive.

### Changed

- `log` 0.4.35 (Dependabot #45).
- `clap` 4.6.9 and `regex` 1.13.2 (Dependabot #49).

### Fixed

- `normalize_urls` returns URLs in the order each first appears instead
  of hash-set order, so the same input gives the same sitemap on every
  run. Duplicates are still dropped, keeping the first occurrence.

### Removed

- The `euxis-commons` and `time` dependencies. No code in the crate
  used either (`time` still arrives through `dtt`); they only added to
  every build and to the supply-chain surface.

## [0.0.7] - 2026-10-31

### Added

- Tag-driven GitHub Actions release workflow (`.github/workflows/release.yml`)
  that runs `./scripts/release-preflight.sh`, cross-platform CI, cross-compiled
  binary builds, SLSA provenance attestation, SPDX SBOM generation, and
  crates.io publishing via OIDC Trusted Publishing.

### Changed

- **Replaced `quick-xml` with `oxml` 0.0.13** for memory-safe, zero-`unsafe`
  W3C XML 1.0 sitemap serialization and parsing.
- **Breaking: the MSRV is now Rust 1.89.0** (was 1.80.0), one floor
  shared with the maintainer's other Rust crates. `rust-version` in
  `Cargo.toml` is the only place sitemap-gen states it.

### Dependencies

- Added `oxml` 0.0.13 and removed `quick-xml` 0.39.
- The minor-and-patch dependency group (Dependabot #40).

## [0.0.2] - 2026-06-21

### Added

- Unified GitHub Actions pipeline (`.github/workflows/ci.yml`) that delegates
  to the shared `sebastienrousseau/pipelines` reusable workflows for CI,
  security, and docs.
- `.github/labeler.yml` for automatic PR labelling (docs, rust, ci, tests,
  dependencies).

### Changed

- **CI consolidation.** Replaced the seven per-concern workflow files
  (`audit.yml`, `check.yml`, `coverage.yml`, `document.yml`, `lint.yml`,
  `release.yml`, `test.yml`) with a single `ci.yml` that fans out to the
  shared pipeline. Concurrency is now gated per-ref to cancel superseded
  runs.
- **Dependabot configuration.** Cadence moved from daily to weekly (Mondays
  at 09:00 UTC), grouped minor/patch updates for the Cargo ecosystem, raised
  the open-PR cap, added labels, and standardised the `chore(deps)` commit
  prefix.
- **Release profile.** `opt-level` raised from `"s"` to `3` to prioritise
  runtime performance over binary size for the published artefact.
- **README.** Rewritten with a centred layout, refreshed badges (now pointing
  at the unified `ci.yml`), corrected install snippets to `0.0.2`, and a
  working `Usage` example that matches the current public API
  (`Sitemap` / `SiteMapData` / `ChangeFreq`).
- Replaced `"".to_string()` with `String::new()` in `utils::format_date` for
  a minor allocation tidy.

### Dependencies

- `scraper` 0.22 -> 0.23 (supersedes #10).
- `criterion` 0.5 -> 0.6 (supersedes #11). Updated the benchmark to import
  `black_box` from `std::hint` per the criterion 0.6 deprecation notice.
- Refreshed `Cargo.lock` via `cargo update` (transitive bumps only).

## [0.0.1] - 2024-10

- Initial public release.
