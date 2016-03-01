extern crate time;
use time::{Duration, SteadyTime};
use std::fmt;

pub struct TimeResult(i64, i64);

pub struct Bencher {
    iterations: u32,
    data: Vec<i64>
}

impl fmt::Display for TimeResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} +/- {} µs", self.0, self.1)
    }
}

fn avg(data: &Vec<i64>) -> i64 {
    let avg: i64 = data.iter()
                       .fold(0, |acc, x| acc + x);
    avg / data.len() as i64
}

impl Bencher {
    pub fn new(iterations: u32) -> Bencher {
        Bencher {
            iterations: iterations,
            data: Vec::new()
        }
    }
    fn results(&self) -> TimeResult {
        let average: i64 = avg(&self.data);
        let deviations: Vec<i64> = self.data.iter()
                                            .map(|x| (x - average).pow(2))
                                            .collect();
        let dispersion: i64 = avg(&deviations);
        TimeResult(average, (dispersion as f64).sqrt().round() as i64)
    }
    fn recollect(&mut self, data: Vec<Duration>) {
        self.data = data.iter()
                        .filter_map(|x| x.num_microseconds())
                        .collect();
    }
    pub fn s_bench<F>(&mut self, f: F) -> TimeResult
        where F: Fn() + 'static
    {
        let mut intervals = Vec::new();
        for _ in 0..self.iterations {
            let start = SteadyTime::now();
            f();
            let stop = SteadyTime::now();
            intervals.push(stop - start);
        }
        self.recollect(intervals);
        self.results()
    }
    pub fn m_bench<F>(&mut self, f: &mut F) -> TimeResult
        where F: FnMut() + 'static
    {
        let mut intervals = Vec::new();
        for _ in 0..self.iterations {
            let start = SteadyTime::now();
            f();
            let stop = SteadyTime::now();
            intervals.push(stop - start);
        }
        self.recollect(intervals);
        self.results()
    }
}
