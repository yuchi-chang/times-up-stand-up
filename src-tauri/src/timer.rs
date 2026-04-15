use std::sync::Mutex;
use std::time::{Duration, Instant};

pub struct TimerState {
    pub interval_secs: u64,
    pub target: Option<Instant>,
    pub running: bool,
}

impl TimerState {
    pub fn new(interval_secs: u64) -> Self {
        Self {
            interval_secs,
            target: Some(Instant::now() + Duration::from_secs(interval_secs)),
            running: true,
        }
    }

    pub fn remaining_secs(&self) -> u64 {
        if !self.running {
            return self.interval_secs;
        }
        match self.target {
            Some(target) => {
                let now = Instant::now();
                if now >= target {
                    0
                } else {
                    (target - now).as_secs()
                }
            }
            None => self.interval_secs,
        }
    }

    pub fn reset(&mut self) {
        self.target = Some(Instant::now() + Duration::from_secs(self.interval_secs));
        self.running = true;
    }

    pub fn toggle_pause(&mut self) -> bool {
        if self.running {
            // Pause: store remaining time in interval_secs temporarily
            self.interval_secs = self.remaining_secs();
            self.target = None;
            self.running = false;
        } else {
            // Resume: set new target from stored remaining time
            self.target = Some(Instant::now() + Duration::from_secs(self.interval_secs));
            self.running = true;
        }
        self.running
    }

    pub fn is_expired(&self) -> bool {
        self.running && self.remaining_secs() == 0
    }

    pub fn update_interval(&mut self, new_interval_secs: u64) {
        self.interval_secs = new_interval_secs;
        self.reset();
    }
}

pub type TimerMutex = Mutex<TimerState>;
