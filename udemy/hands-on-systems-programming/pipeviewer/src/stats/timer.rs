use std::time::{Duration, Instant};

pub struct Timer {
    pub last_instant: Instant,
    pub delta: Duration,
    pub period: Duration,
    pub countdown: Duration,
    pub ready: bool,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            last_instant: Instant::now(),
            delta: Duration::default(),
            period: Duration::from_millis(1000),
            countdown: Duration::default(),
            ready: true,
        }
    }
    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_instant;
        self.last_instant = now;
        self.countdown = self.countdown.checked_sub(self.delta).unwrap_or_else(|| {
            self.ready = true;
            self.period
        })
    }
}

pub trait TimeOutput {
    fn as_time(&self) -> String;
}

impl TimeOutput for u64 {
    fn as_time(&self) -> String {
        let (hours, left) = (*self / 3600, *self % 3600);
        let (minutes, seconds) = (left / 60, left % 60);
        format!("Time: {}:{:02}:{:02}; ", hours, minutes, seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::TimeOutput;
    #[test]
    fn as_time_format() {
        let pairs = vec![
            (5_u64, "Time: 0:00:05; "),
            (60_u64, "Time: 0:01:00; "),
            (154_u64, "Time: 0:02:34; "),
            (3603_u64, "Time: 1:00:03; "),
        ];
        for (input, output) in pairs {
            assert_eq!(input.as_time().as_str(), output);
        }
    }
}
