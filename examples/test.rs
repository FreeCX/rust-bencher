extern crate rust_bencher as bencher;
use bencher::Bencher;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut b = Bencher::new(1000);
    let result = b.s_bench(|| {
        sleep(Duration::new(0, 5_000_000));
    });
    println!("result: {}", result);
}
