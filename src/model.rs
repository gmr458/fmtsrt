use crate::util;
use std::{io, time};

#[derive(Debug)]
pub struct Subtitle {
    pub number: usize,
    pub start: TimeCode,
    pub end: TimeCode,
    pub text: String,
}

impl Subtitle {
    pub fn to_string(&self) -> String {
        format!(
            r#"{}
{} --> {}
{}
"#,
            self.number,
            self.start.to_string(),
            self.end.to_string(),
            self.text.trim_start(),
        )
    }
}

#[derive(Debug)]
pub struct TimeCode {
    pub hours: u64,
    pub mins: u64,
    pub secs: u64,
    pub millis: u32,
    pub duration: time::Duration,
}

impl TimeCode {
    pub fn new() -> TimeCode {
        let duration = time::Duration::new(0, 0);

        TimeCode {
            hours: 0,
            mins: 0,
            secs: 0,
            millis: 0,
            duration,
        }
    }

    pub fn build(hours: u64, mins: u64, secs: u64, millis: u32) -> TimeCode {
        let duration = time::Duration::new(
            util::hours_to_secs(hours) + util::mins_to_secs(mins) + secs,
            util::millis_to_nanos(millis),
        );

        TimeCode {
            hours,
            mins,
            secs,
            millis,
            duration,
        }
    }

    fn to_string(&self) -> String {
        format!(
            "{:02}:{:02}:{:02},{:03}",
            self.hours, self.mins, self.secs, self.millis
        )
    }

    pub fn add_secs(&mut self, secs: u64) {
        let duration_to_add = time::Duration::new(secs, 0);
        self.duration = self.duration.checked_add(duration_to_add).unwrap();

        let ss = self.duration.as_secs();

        let (hh, ss) = (ss / 3600, ss % 3600);
        let (mm, ss) = (ss / 60, ss % 60);

        self.hours = hh;
        self.mins = mm;
        self.secs = ss;
    }

    pub fn sub_secs(&mut self, secs: u64) -> std::io::Result<()> {
        let duration_to_sub = time::Duration::new(secs, 0);

        match self.duration.checked_sub(duration_to_sub) {
            Some(new_duration) => {
                self.duration = new_duration;

                let ss = self.duration.as_secs();

                let (hh, ss) = (ss / 3600, ss % 3600);
                let (mm, ss) = (ss / 60, ss % 60);

                self.hours = hh;
                self.mins = mm;
                self.secs = ss;

                Ok(())
            }
            None => {
                let error = io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid amount of seconds to subtract",
                );

                Err(error)
            }
        }
    }
}

#[derive(Debug)]
pub enum LineKind {
    Number,
    TimeCodes,
    Text,
    Empty,
}

#[derive(Debug)]
pub struct LineTracker {
    pub prev: Option<LineKind>,
    pub next: LineKind,
}
