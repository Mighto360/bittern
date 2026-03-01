use divan::Bencher;
use bittern::Arena;

fn main() {
    divan::main();
}

#[divan::bench(sample_size = 1000)]
fn compare_symbols(b: Bencher) {
    // Setup
    let arena: Arena<str> = Arena::new();
    let s1 = arena.intern("hello world");
    let s2 = arena.intern("hello world");
    // Benchmark
    b.bench_local(|| {
        assert!(s1.is(&s2));
    });
}

#[divan::bench(sample_size = 1000)]
fn compare_strings(b: Bencher) {
    // Setup
    let s1 = "hello world";
    let s2 = "hello world";
    // Benchmark
    b.bench_local(|| {
        assert_eq!(s1, s2);
    });
}
