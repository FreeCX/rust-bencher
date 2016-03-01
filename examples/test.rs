extern crate rust_bencher as bencher;
use bencher::Bencher;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut b = Bencher::new(10);
    let (a, b) = b.s_bench(|| {
        sleep(Duration::new(0, 2000));
    });
    println!("result: {} +/- {} µs", a, b);
}