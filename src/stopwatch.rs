//! # Stopwatch
//!
//! A stopwatch that mimics iOS's stopwatch.
//!
//! ## Usage
//!
//! - Use `Stopwatch::new()` to initialise a new stopwatch instance. The stopwatch is paused
//! at `00:00` and will **not** run until you call `.resume()` or `.pause_or_resume()`.
//! - While running:
//!     - Call `.lap()` to record lap times.
//!     - Call `.pause_or_resume()`, `.pause()` or `.resume()` to pause or resume.
//! - When you want to stop (reset), call `.stop()`, which resets the stopwatch and returns data
//!
//! ## Examples
//!
//! ## Schematic
//!
//! ```ignore
//!                  lap    lap          lap
//! start       start |      |     start  |
//!   o--------x   o-----------x      o-----------x
//!          pause           pause            pause(end)
//! ```

use chrono::{DateTime, Duration, Local};
use std::mem;

#[derive(Debug)]
pub struct StopwatchData {
    pub elapsed: Duration,
    pub pause_moments: Vec<DateTime<Local>>, // moments at which the stopwatch is paused
    pub start_moments: Vec<DateTime<Local>>, // moments at which the stopwatch resumes
    pub lap_moments: Vec<DateTime<Local>>,   // moments at which a lap time is read
    pub laps: Vec<Duration>,                 // lap times
}

impl StopwatchData {
    pub fn new() -> Self {
        Self {
            elapsed: Duration::zero(),
            start_moments: Vec::new(),
            pause_moments: Vec::new(),
            lap_moments: Vec::new(),
            laps: Vec::new(),
        }
    }
    pub fn start(&self) -> DateTime<Local> {
        self.start_moments[0]
    }
    pub fn stop(&self) -> DateTime<Local> {
        self.pause_moments[self.pause_moments.len() - 1]
    }
}

#[derive(Debug)]
pub struct Stopwatch {
    pub lap_elapsed: Duration, // elapsed time of the current lap
    pub paused: bool,
    pub data: StopwatchData,
}

impl Stopwatch {
    /// Returns stopwatch reset to zero
    pub fn new() -> Self {
        Self {
            lap_elapsed: Duration::zero(),
            paused: true, // stopped by default; start by explicitly calling `.resume()`
            data: StopwatchData::new(),
        }
    }
    /// Read the total time elapsed
    pub fn read(&self) -> Duration {
        if self.paused {
            self.data.elapsed
        } else {
            self.data.elapsed + (Local::now() - self.last_start())
        }
    }
    pub fn pause_or_resume(&mut self) {
        if self.paused {
            self.resume();
        } else {
            self.pause();
        }
    }
    pub fn lap(&mut self) -> Option<Duration> {
        // assert!(!self.paused, "Paused!");
        if self.paused {
            None
        } else {
            let moment = Local::now();
            let lap = self.read_lap_elapsed(moment);
            self.data.lap_moments.push(moment);
            self.data.laps.push(lap);
            self.lap_elapsed = Duration::zero();
            Some(lap)
        }
    }
    /// Read the time elapsed in the current lap
    fn read_lap_elapsed(&self, moment: DateTime<Local>) -> Duration {
        self.lap_elapsed
            + if self.lap_elapsed == Duration::zero() && !self.data.lap_moments.is_empty() {
                moment - self.last_lap()
            } else {
                moment - self.last_start()
            }
    }

    fn last_start(&self) -> DateTime<Local> {
        self.data.start_moments[self.data.start_moments.len() - 1]
    }
    fn last_lap(&self) -> DateTime<Local> {
        self.data.lap_moments[self.data.lap_moments.len() - 1]
    }
    pub fn pause(&mut self) {
        let moment = Local::now();
        self.data.pause_moments.push(moment);
        self.data.elapsed = self.data.elapsed + (moment - self.last_start());
        self.lap_elapsed = self.read_lap_elapsed(moment);
        self.paused = true;
    }
    pub fn resume(&mut self) {
        self.data.start_moments.push(Local::now());
        self.paused = false;
    }
    pub fn stop(&mut self) -> StopwatchData {
        let moment = Local::now();
        // lap
        let lap = self.read_lap_elapsed(moment);
        self.data.lap_moments.push(moment);
        self.data.laps.push(lap);
        self.lap_elapsed = Duration::zero();
        // pause
        self.data.pause_moments.push(moment);
        self.data.elapsed = self.data.elapsed + (moment - self.last_start());
        self.paused = true;
        // data
        let data = mem::replace(&mut self.data, StopwatchData::new());
        data
    }
}
