#![allow(missing_docs)]

use criterion::{
    black_box, criterion_group, criterion_main, BatchSize, Criterion,
};
use sitemap_gen::{ChangeFreq, SiteMapData, Sitemap};
use url::Url;

fn generate_sitemap(n: usize) -> Sitemap {
    let mut sitemap = Sitemap::new();
    let base_url = "https://example.com/page";

    for i in 0..n {
        let entry = SiteMapData {
            loc: Url::parse(&format!("{}{}", base_url, i)).unwrap(),
            lastmod: "2023-05-20".to_string(),
            changefreq: ChangeFreq::Weekly,
        };
        sitemap.add_entry(entry).expect("Failed to add entry");
    }
    sitemap
}

fn benchmark_sitemap_generation(c: &mut Criterion) {
    let _ = c.bench_function("sitemap_generation", |b| {
        // Create a static sitemap size to benchmark against
        b.iter_batched(
            || 1000, // Input size
            generate_sitemap,
            BatchSize::SmallInput,
        );
    });
}

fn benchmark_sitemap_serialization(c: &mut Criterion) {
    let _ = c.bench_function("sitemap_serialization", |b| {
        // Pre-generate a sitemap for serialization to avoid benchmarking the generation
        let sitemap = generate_sitemap(1000);

        b.iter_batched(
            || sitemap.clone(),
            |sitemap| {
                let xml = sitemap.to_xml();
                let _ = black_box(xml);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(
    benches,
    benchmark_sitemap_generation,
    benchmark_sitemap_serialization
);
criterion_main!(benches);
