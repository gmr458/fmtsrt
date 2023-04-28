use crate::model;
use std::io;

pub fn reset_nums(subs: &mut Vec<model::Subtitle>) {
    let mut num = 1;

    for sub in subs {
        sub.number = num;
        num += 1;
    }
}

pub fn add_secs(subs: &mut Vec<model::Subtitle>, secs: u64) {
    for sub in subs {
        sub.start.add_secs(secs);
        sub.end.add_secs(secs);
    }
}

pub fn sub_secs(subs: &mut Vec<model::Subtitle>, secs: u64) -> io::Result<()> {
    for sub in subs {
        sub.start.sub_secs(secs)?;
        sub.end.sub_secs(secs)?;
    }

    Ok(())
}
