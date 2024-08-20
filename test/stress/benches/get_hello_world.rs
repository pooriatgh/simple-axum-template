use criterion::{black_box, criterion_group, criterion_main, Criterion};

async fn get_hello_world(n: u64) -> () {
    println!("Hello, world! for {} times", n);
    let hc = httpc_test::new_client("http://localhost:3000").unwrap();
    let _ = hc.do_get("/api/hello").await;
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("hello world", |b| b.iter(|| get_hello_world(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
