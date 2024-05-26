use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::Throughput;
use huffman_coding::huffman_encode;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn bench_huffman_encode(ctx: &mut Criterion) {
    let mut group = ctx.benchmark_group("Huffman Encode");
    let step = 1024;
    for size in std::iter::successors(Some(step), |x| Some(x + step)).take(20) {
        let data: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(size)
            .map(|c| c as char)
            .collect();
        group.throughput(Throughput::Bytes(data.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("huffman_encode", data.len()),
            &data,
            |b, text| {
                b.iter(|| huffman_encode(&text));
            },
        );
    }
}

criterion_group!(benches, bench_huffman_encode);
criterion_main!(benches);
