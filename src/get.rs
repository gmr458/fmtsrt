use crate::{model, validate};
use std::process;

/// Take the lines that came from the .srt file and return a vector of Subtitle for easy data
/// manipulation.
///
/// # Example
///
/// ```
/// use fmtsrt::model::{Subtitle, TimeCode};
/// use fmtsrt::get::subs_from_lines;
///
/// let srt_file_content = r#"1
/// 00:00:01,040 --> 00:00:04,000
/// First subtitle
///
/// 2
/// 00:00:04,000 --> 00:00:08,720
/// seconds subtitle,
/// and a third subtitle.
/// "#;
///
/// let lines: Vec<&str> = srt_file_content.split("\n").collect();
/// let subs = subs_from_lines(lines);
///
/// assert_eq!(subs.len(), 2);
///
/// assert_eq!(subs[0].number, 1);
/// assert_eq!(subs[0].start.hours, 0);
/// assert_eq!(subs[0].start.mins, 0);
/// assert_eq!(subs[0].start.secs, 1);
/// assert_eq!(subs[0].start.millis, 40);
/// assert_eq!(subs[0].end.hours, 0);
/// assert_eq!(subs[0].end.mins, 0);
/// assert_eq!(subs[0].end.secs, 4);
/// assert_eq!(subs[0].end.millis, 0);
/// assert_eq!(subs[0].text, String::from("\nFirst subtitle"));
///
/// assert_eq!(subs[1].number, 2);
/// assert_eq!(subs[1].start.hours, 0);
/// assert_eq!(subs[1].start.mins, 0);
/// assert_eq!(subs[1].start.secs, 4);
/// assert_eq!(subs[1].start.millis, 0);
/// assert_eq!(subs[1].end.hours, 0);
/// assert_eq!(subs[1].end.mins, 0);
/// assert_eq!(subs[1].end.secs, 8);
/// assert_eq!(subs[1].end.millis, 720);
/// assert_eq!(subs[1].text, String::from("\nseconds subtitle,\nand a third subtitle."));
/// ```
pub fn subs_from_lines(lines: Vec<&str>) -> Vec<model::Subtitle> {
    let mut subs: Vec<model::Subtitle> = Vec::new();

    let mut number: usize = 0;
    let mut start = model::TimeCode::new();
    let mut end = model::TimeCode::new();
    let mut text = String::new();

    let mut tracker = model::LineTracker {
        prev: None,
        next: model::LineKind::Number,
    };

    let mut number_line: u16 = 1;

    for line in lines {
        match tracker.next {
            model::LineKind::Number => {
                if line.is_empty() {
                    match tracker.prev {
                        Some(model::LineKind::Empty) => {
                            break;
                        }
                        _ => {}
                    }
                }

                // Text files with `UTF-8 with BOM` encoding have the string `<feff>`
                // at the beginning of its content, it should be removed
                let line = line.strip_prefix("﻿").unwrap_or(line);

                if validate::num(line) {
                    number = line.trim().parse().unwrap();
                    tracker.prev = Some(model::LineKind::Number);
                    tracker.next = model::LineKind::TimeCodes;
                } else {
                    eprintln!("Incorrect number in .srt file at line {}", number_line);
                    process::exit(1);
                }
            }
            model::LineKind::TimeCodes => {
                if validate::timecodes(line) {
                    let timecodes: Vec<&str> = line.split(" --> ").collect();

                    let start_timecode = timecodes[0];
                    let (hh, mm, ss, ms) = hh_mm_ss_ms(start_timecode);
                    start = model::TimeCode::build(hh, mm, ss, ms);

                    let end_timecode = timecodes[1];
                    let (hh, mm, ss, ms) = hh_mm_ss_ms(end_timecode);
                    end = model::TimeCode::build(hh, mm, ss, ms);

                    tracker.prev = Some(model::LineKind::TimeCodes);
                    tracker.next = model::LineKind::Text;
                } else {
                    eprintln!("Incorrect timecodes in .srt file at line {}", number_line);
                    process::exit(1);
                }
            }
            model::LineKind::Text => {
                if line.is_empty() {
                    match tracker.prev {
                        Some(model::LineKind::TimeCodes) => {
                            eprintln!("Text not found in .srt file at line {}", number_line);
                            process::exit(1);
                        }
                        Some(model::LineKind::Text) => {
                            let sub = model::Subtitle {
                                number,
                                start,
                                end,
                                text,
                            };

                            subs.push(sub);

                            number = 0;
                            start = model::TimeCode::new();
                            end = model::TimeCode::new();
                            text = String::from("");

                            tracker.prev = Some(model::LineKind::Empty);
                            tracker.next = model::LineKind::Number;
                        }
                        _ => {}
                    }
                } else {
                    text = format!("{text}\n{}", line);
                    tracker.prev = Some(model::LineKind::Text);
                    tracker.next = model::LineKind::Text;
                }
            }
            _ => {}
        }

        number_line += 1;
    }

    subs
}

/// Get hours, minutes, seconds and milliseconds from a string literal with the format hh::mm::ss,xxx
pub fn hh_mm_ss_ms(timecode: &str) -> (u64, u64, u64, u32) {
    let values: Vec<&str> = timecode.split(":").collect();

    let hh = values[0].trim().parse().unwrap();
    let mm = values[1].trim().parse().unwrap();

    let ss_and_ms: Vec<&str> = values[2].split(",").collect();
    let ss: u64 = ss_and_ms[0].trim().parse().unwrap();
    let ms: u32 = ss_and_ms[1].trim().parse().unwrap();

    (hh, mm, ss, ms)
}

pub fn text_contents_subs(subs: Vec<model::Subtitle>) -> String {
    let mut text = String::new();

    for sub in subs {
        text = format!("{}\n{}", text.trim_start(), sub.to_string());
    }

    text = format!("{}\n", text);

    text
}
