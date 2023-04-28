use crate::{cli, model};
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

pub fn print_change_applied(command: cli::Commands, secs: u64) {
    let mut secs_string = "second";
    if secs > 1 {
        secs_string = "seconds";
    }

    match command {
        cli::Commands::Add { .. } => {
            println!("{} {} added", secs, secs_string);
        }
        cli::Commands::Subtract { .. } => {
            println!("{} {} subtracted", secs, secs_string);
        }
    }
}

pub fn sub_secs(subs: &mut Vec<model::Subtitle>, secs: u64) -> io::Result<()> {
    for sub in subs {
        sub.start.sub_secs(secs)?;
        sub.end.sub_secs(secs)?;
    }

    Ok(())
}
