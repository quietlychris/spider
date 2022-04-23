use criterion::{criterion_group, black_box, criterion_main, Criterion};
use std::process::{Command};
use std::time::Duration;
pub mod node_crawler;
pub mod go_crolly;

/// bench spider crawling between different libs
pub fn bench_speed(c: &mut Criterion) {
    let mut group = c.benchmark_group("crawl-speed/libraries");
        
    group.sample_size(10).measurement_time(Duration::new(180, 0) + Duration::from_millis(500));
    group.bench_function("Rust[spider]: with crawl 10 times", |b| b.iter(||black_box(Command::new("spider")
        .args(["--delay", "0", "--domain", "https://rsseau.fr", "crawl"])
        .output()
        .expect("rust command failed to start"))
    ));
    group.bench_function("Go[crolly]: with crawl 10 times", |b| b.iter(||black_box(Command::new("./gospider")
        .output()
        .expect("go command failed to start"))
    ));
    group.bench_function("Node.js[crawler]: with crawl 10 times", |b| b.iter(|| black_box(Command::new("node")
        .arg("./node-crawler.js")
        .output()
        .expect("node command failed to start"))
    ));
    group.finish();
}


/// bench spider crawling stack vs while
pub fn bench_speed_internal(c: &mut Criterion) {
    let mut group = c.benchmark_group("crawl-speed/internal");
    
    let crawl_args = ["--delay", "0", "--domain", "https://rsseau.fr", "crawl"];
    let crawl_args_stack = ["--delay", "0", "--domain", "https://rsseau.fr", "crawl", "--stack"];

    group.sample_size(10).measurement_time(Duration::new(180, 0) + Duration::from_millis(500));
    group.bench_function("spider[crawl]: with crawl 10 times", |b| b.iter(||black_box(Command::new("spider")
        .args(crawl_args)
        .output()
        .expect("rust command failed to start"))
    ));
    group.bench_function("spider[crawl_stack]: with crawl 10 times", |b| b.iter(||black_box(Command::new("spider")
        .args(crawl_args_stack)
        .output()
        .expect("rust command failed to start"))
    ));
    group.finish();
}

criterion_group!(benches, bench_speed_internal, bench_speed);
criterion_main!(benches);
