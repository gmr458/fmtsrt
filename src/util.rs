pub fn hours_to_secs(hours: u64) -> u64 {
    hours * 3600
}

pub fn mins_to_secs(mins: u64) -> u64 {
    mins * 60
}

pub fn millis_to_nanos(millis: u32) -> u32 {
    millis * 1_000_000
}
