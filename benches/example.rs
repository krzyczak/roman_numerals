#[macro_use]
extern crate bencher;

#[path = "../src/numerals.rs"]
mod numerals;

use bencher::Bencher;

fn bench_arabic_to_roman(bench: &mut Bencher) {
    // bench.iter(|| main::arabic_to_roman(3457));
    bench.iter(|| numerals::arabic_to_roman(1776));
}

benchmark_group!(benches, bench_arabic_to_roman);
benchmark_main!(benches);
