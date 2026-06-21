# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
