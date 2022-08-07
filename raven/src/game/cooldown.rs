// modified from https://github.com/kokounet/bevy_time/blob/main/src/cooldown.rs
// TODO: I'm pretty sure this can now be done with Timer
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
    pub fn is_available(&self) -> bool {
        self.available
    }

    #[inline]
    pub fn is_just_available(&self) -> bool {
        self.just_available
    }

    #[inline]
    pub fn get_elapsed(&self) -> f32 {
        self.stopwatch.elapsed().as_secs_f32()
    }

    #[inline]
    pub fn get_duration(&self) -> f32 {
        self.duration
    }

    #[inline]
    pub fn set_duration(&mut self, duration: f32) {
        self.duration = duration;
    }

    pub fn tick(&mut self, delta: f32) -> &Self {
        if self.is_available() {
            self.just_available = false;
            return self;
        }

        self.stopwatch.tick(Duration::from_secs_f32(delta));
        if self.get_elapsed() >= self.get_duration() {
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
