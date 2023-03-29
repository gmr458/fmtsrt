pub fn num(line: &str) -> bool {
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

pub fn timecodes(line: &str) -> bool {
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
