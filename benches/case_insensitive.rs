use fsearch::search::SearchBuilder;

use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_search_1(c: &mut Criterion) {
    let search = SearchBuilder::from_pattern("Lorem")
        .case_sensitive(false)
        .build();
    c.bench_function("optimized search 1", |b| {
        b.iter(|| {
            let content = "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.";
            let _ = search.search(content);
        });
    });
}

fn benchmark_search_2(c: &mut Criterion) {
    let search = SearchBuilder::from_pattern("zzzzzzzzzzzzzzzzzzz")
        .case_sensitive(false)
        .build();
    let content = "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.";
    c.bench_function("optimized search 2", |b| {
        b.iter(|| {
            let _ = search.search(content);
        });
    });
}

fn benchmark_normal_search_1(c: &mut Criterion) {
    let search = SearchBuilder::from_pattern("Lorem")
        .case_sensitive(false)
        .build();
    c.bench_function("normal search 1", |b| {
        b.iter(|| {
            let content = "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.";
            let _ = search.naive_search(content);
        });
    });
}

fn benchmark_normal_search_2(c: &mut Criterion) {
    let search = SearchBuilder::from_pattern("zzzzzzzzzzzzzzzzzzz")
        .case_sensitive(false)
        .build();
    let content = "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.";
    c.bench_function("normal search 2", |b| {
        b.iter(|| {
            let _ = search.naive_search(content);
        });
    });
}

criterion_group!(
    case_insensitive_benches,
    benchmark_search_1,
    benchmark_search_2,
    benchmark_normal_search_1,
    benchmark_normal_search_2
);
criterion_main!(case_insensitive_benches);
