use crate::model;
use std::{fs, io};

pub fn reset_nums(subs: &mut Vec<model::Subtitle>) {
    let mut num = 1;

    for sub in subs {
        sub.number = num;
        num += 1;
    }
}

pub fn print_subs(subs: Vec<model::Subtitle>) {
    for sub in subs {
        println!("{}", sub.to_string());
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

pub fn create_outpur_dir(dirname: String) -> io::Result<()> {
    fs::create_dir_all(dirname)?;
    Ok(())
}

pub fn write_content_in_file(output_file: String, contents: String) -> io::Result<()> {
    fs::write(output_file, contents)?;
    Ok(())
}
