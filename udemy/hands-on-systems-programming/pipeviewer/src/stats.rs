use crossbeam::channel::Receiver;
use std::io::Result;
use std::time::{Duration, Instant};

struct Timer {
    last_instant: Instant,
    delta: Duration,
    period: Duration,
    countdown: Duration,
    ready: bool,
}

impl Timer {
    fn new() -> Self {
        Self {
            last_instant: Instant::now(),
            delta: Duration::default(),
            period: Duration::from_millis(1000),
            countdown: Duration::default(),
            ready: true,
        }
    }
    fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_instant;
        self.last_instant = now;
        self.countdown = self.countdown.checked_sub(self.delta).unwrap_or_else(|| {
            self.ready = true;
            self.period
        })
    }
}

trait TimeOutput {
    fn as_time(&self) -> String;
}

impl TimeOutput for u64 {
    fn as_time(&self) -> String {
        let (hours, left) = (*self / 3600, *self % 3600);
        let (minutes, seconds) = (self / 60, left % 60);
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    }
}

pub fn stats(verbose: bool, total_bytes: usize, last: bool) {
    if verbose {
        eprint!("\r{}", total_bytes);
        if last {
            eprintln!();
        }
    }
}

pub fn stats_loop(verbose: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;
    let start = Instant::now();
    let mut timer = Timer::new();
    loop {
        let num_bytes = stats_rx.recv().unwrap();
        timer.update();
        let rate_per_second = num_bytes as f64 / timer.delta.as_secs_f64();
        total_bytes += num_bytes;
        if verbose && timer.ready {
            timer.ready = false;
            eprint!(
                "\r{} bytes in {} seconds [{:.0} b/s]",
                total_bytes,
                start.elapsed().as_secs().as_time(),
                rate_per_second
            );
        }
        if num_bytes == 0 {
            break;
        }
    }
    if verbose {
        eprintln!();
    }
    Ok(())
}
