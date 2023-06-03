#[macro_use]
extern crate bencher;

#[path = "../src/numerals.rs"]
mod numerals;

use bencher::Bencher;

fn bench_arabic_to_roman(bench: &mut Bencher) {
    // bench.iter(|| main::arabic_to_roman(3457));
    bench.iter(|| numerals::arabic_to_roman(1776));
}

fn bench_roman_to_arabic(bench: &mut Bencher) {
    bench.iter(|| numerals::roman_to_arabic("MMMCDLVI".into()));
}

benchmark_group!(benches, bench_arabic_to_roman, bench_roman_to_arabic);
benchmark_main!(benches);
