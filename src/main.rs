use std::fs;
use std::time;

fn main() {
    let input = fs::read_to_string("./examples/the_real_dune.srt").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    let mut subtitles: Vec<Subtitle> = Vec::new();

    let mut num: i32 = 0;

    let mut start_in_seconds: u64 = 0;
    let mut start_hundredths: u16 = 0;

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
                start: TimeCode::new(start_in_seconds, start_hundredths),
                end: TimeCode::new(end_in_seconds, end_ms),
                text,
            });

            num = 0;
            start_in_seconds = 0;
            start_hundredths = 0;
            end_in_seconds = 0;
            end_ms = 0;
            text = String::from("");
            continue;
        }

        if is_num(line) {
            num = line.trim().parse().unwrap();
            continue;
        }

        if is_timecodes(line) {
            let timecodes: Vec<&str> = line.split(" --> ").collect();

            let start = timecodes[0];
            let end = timecodes[1];

            let values: Vec<&str> = start.split(":").collect();

            let hour = values[0];
            let hour: u64 = hour.trim().parse().unwrap();

            let minute = values[1];
            let minute: u64 = minute.trim().parse().unwrap();

            let second_hundredths: Vec<&str> = values[2].split(",").collect();

            let second = second_hundredths[0];
            let second: u64 = second.trim().parse().unwrap();
            let hundredths = second_hundredths[1];

            start_in_seconds = hours_to_seconds(hour) + minutes_to_seconds(minute) + second;
            start_hundredths = hundredths.trim().parse().unwrap();

            // -----------------------------

            let values: Vec<&str> = end.split(":").collect();

            let hh = values[0];
            let hh: u64 = hh.trim().parse().unwrap();

            let mm = values[1];
            let mm: u64 = mm.trim().parse().unwrap();

            let ss_ms: Vec<&str> = values[2].split(",").collect();

            let ss = ss_ms[0];
            let ss: u64 = ss.trim().parse().unwrap();

            let ms = ss_ms[1];

            end_in_seconds = hours_to_seconds(hh) + minutes_to_seconds(mm) + ss;
            end_ms = ms.trim().parse().unwrap();

            continue;
        }

        text = format!("{text}\n{}", line);
    }

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

fn is_timecodes(line: &str) -> bool {
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
