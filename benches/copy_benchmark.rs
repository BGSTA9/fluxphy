//! Benchmark for FluxPhy copy performance

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::fs::{self, File};
use std::io::Write;
use tempfile::TempDir;

fn create_test_file(size_mb: usize) -> (TempDir, std::path::PathBuf) {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("test_file.bin");

    let mut file = File::create(&path).unwrap();
    let chunk: Vec<u8> = (0..1024 * 1024).map(|i| (i % 256) as u8).collect();

    for _ in 0..size_mb {
        file.write_all(&chunk).unwrap();
    }
    file.flush().unwrap();

    (temp, path)
}

fn benchmark_copy_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_copy");

    for size in [1, 10].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}MB", size)),
            size,
            |b, &size| {
                let (temp, source) = create_test_file(size);
                let dest = temp.path().join("dest.bin");

                b.iter(|| {
                    fs::copy(black_box(&source), black_box(&dest)).unwrap();
                    fs::remove_file(&dest).ok();
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, benchmark_copy_performance);
criterion_main!(benches);
