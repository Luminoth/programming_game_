// modified from https://github.com/kokounet/bevy_time/blob/main/src/cooldown.rs
#![allow(dead_code)]

use std::time::Duration;

use bevy::time::Stopwatch;

#[derive(Debug, Clone)]
pub struct Cooldown {
    stopwatch: Stopwatch,
    duration: f32,
    available: bool,
    just_available: bool,
}

impl Cooldown {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration: duration.as_secs_f32(),
            stopwatch: Default::default(),
            available: true,
            just_available: true,
        }
    }

    pub fn from_seconds(duration: f32) -> Self {
        Self {
            duration,
            stopwatch: Default::default(),
            available: true,
            just_available: true,
        }
    }

    #[inline]
    pub fn start(&mut self) {
        self.available = false;
        self.just_available = false;
    }

    #[inline]
    pub fn available(&self) -> bool {
        self.available
    }

    #[inline]
    pub fn just_available(&self) -> bool {
        self.just_available
    }

    #[inline]
    pub fn elapsed(&self) -> f32 {
        self.stopwatch.elapsed().as_secs_f32()
    }

    #[inline]
    pub fn duration(&self) -> f32 {
        self.duration
    }

    pub fn tick(&mut self, delta: f32) -> &Self {
        if self.available() {
            self.just_available = false;
            return self;
        }

        self.stopwatch.tick(Duration::from_secs_f32(delta));
        if self.elapsed() >= self.duration() {
            self.reset();
        }

        self
    }

    pub fn reset(&mut self) {
        self.stopwatch.reset();
        self.available = true;
        self.just_available = true;
    }
}
