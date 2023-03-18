use std::fs;
use std::time;

fn main() {
    let input = fs::read_to_string("./examples/the_real_dune.srt").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    let mut subtitles = get_subtitles(lines);

    let more_seconds: u64 = 2;

    for sub in &mut subtitles {
        sub.start.add_seconds(more_seconds);
        sub.end.add_seconds(more_seconds);
    }

    print_subtitles(subtitles);
}

fn get_subtitles(lines: Vec<&str>) -> Vec<Subtitle> {
    let mut subtitles: Vec<Subtitle> = Vec::new();

    let mut num: i32 = 0;

    let mut start_in_seconds: u64 = 0;
    let mut start_ms: u16 = 0;

    let mut end_in_seconds: u64 = 0;
    let mut end_ms: u16 = 0;

    let mut text = String::new();

    for line in lines {
        if line.is_empty() {
            if subtitles.len() > 0 && num <= subtitles.last().unwrap().number {
                continue;
            }

            subtitles.push(Subtitle {
                number: num,
                start: TimeCode::new(start_in_seconds, start_ms),
                end: TimeCode::new(end_in_seconds, end_ms),
                text,
            });

            num = 0;
            start_in_seconds = 0;
            start_ms = 0;
            end_in_seconds = 0;
            end_ms = 0;
            text = String::from("");
            continue;
        }

        if is_num(line) {
            num = line.trim().parse().unwrap();
            continue;
        }

        if are_timecodes(line) {
            let timecodes: Vec<&str> = line.split(" --> ").collect();

            let start = timecodes[0];
            let (hh, mm, ss, ms) = get_hh_mm_ss_ms_from_timecode(start);
            start_in_seconds = hours_to_seconds(hh) + minutes_to_seconds(mm) + ss;
            start_ms = ms;

            let end = timecodes[1];
            let (hh, mm, ss, ms) = get_hh_mm_ss_ms_from_timecode(end);
            end_in_seconds = hours_to_seconds(hh) + minutes_to_seconds(mm) + ss;
            end_ms = ms;

            continue;
        }

        text = format!("{text}\n{}", line);
    }

    subtitles
}

fn print_subtitles(subtitles: Vec<Subtitle>) {
    for subtitle in subtitles {
        println!(
            r#"{}
{} --> {}
{}
"#,
            subtitle.number,
            subtitle.start.to_string(),
            subtitle.end.to_string(),
            subtitle.text.trim_start(),
        );
    }
}

fn is_num(line: &str) -> bool {
    if !line.is_empty()
        && line.chars().next().unwrap().is_numeric()
        && !line.chars().next().unwrap().is_alphabetic()
        && !line.contains(":")
        && !line.contains("-->")
        && !line.contains(",")
    {
        return true;
    }

    false
}

fn are_timecodes(line: &str) -> bool {
    if !line.is_empty()
        && line.chars().next().unwrap().is_numeric()
        && !line.chars().next().unwrap().is_alphabetic()
        && line.contains(":")
        && line.contains("-->")
        && line.contains(",")
    {
        return true;
    }

    false
}

fn get_hh_mm_ss_ms_from_timecode(timecode: &str) -> (u64, u64, u64, u16) {
    let values: Vec<&str> = timecode.split(":").collect();

    let hh = values[0].trim().parse().unwrap();
    let mm = values[1].trim().parse().unwrap();

    let ss_and_ms: Vec<&str> = values[2].split(",").collect();
    let ss: u64 = ss_and_ms[0].trim().parse().unwrap();
    let ms: u16 = ss_and_ms[1].trim().parse().unwrap();

    (hh, mm, ss, ms)
}

fn hours_to_seconds(hours: u64) -> u64 {
    hours * 3600
}

fn minutes_to_seconds(minutes: u64) -> u64 {
    minutes * 60
}

#[derive(Debug)]
struct TimeCode {
    duration: time::Duration,
    ms: u16,
}

impl TimeCode {
    fn new(duration_in_seconds: u64, hundredths: u16) -> TimeCode {
        TimeCode {
            duration: time::Duration::new(duration_in_seconds, 0),
            ms: hundredths,
        }
    }

    fn to_string(&self) -> String {
        let seconds = self.duration.as_secs();

        let (hours, seconds) = (seconds / 3600, seconds % 3600);
        let (minutes, seconds) = (seconds / 60, seconds % 60);

        format!("{:02}:{:02}:{:02},{:03}", hours, minutes, seconds, self.ms)
    }

    fn add_seconds(&mut self, seconds: u64) {
        let more_duration = time::Duration::new(seconds, 0);
        self.duration = self.duration.checked_add(more_duration).unwrap();
    }
}

#[derive(Debug)]
struct Subtitle {
    number: i32,
    start: TimeCode,
    end: TimeCode,
    text: String,
}
