use clap::Parser;
use fmtsrt::{action, cli, get};
use std::{fs, io, path, process};

fn main() -> io::Result<()> {
    let cli = cli::Cli::parse();

    let filepath = cli.input_file.as_deref().unwrap_or_else(|| {
        eprintln!("Use flag --input-file to provide the path of a SRT file");
        process::exit(1);
    });

    let input = match fs::read_to_string(filepath) {
        Ok(input) => input,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                eprintln!("File not found");
                process::exit(1);
            }
            io::ErrorKind::Interrupted => {
                eprintln!("The operation of reading the file was interrupted");
                process::exit(1);
            }
            io::ErrorKind::InvalidData => {
                eprintln!("The file passed is invalid, make sure it is a text file");
                process::exit(1);
            }
            io::ErrorKind::PermissionDenied => {
                eprintln!("Insufficient permissions to read the file");
                process::exit(1);
            }
            _ => {
                eprintln!("Unexpected error, make sure you're passing a text file");
                process::exit(1);
            }
        },
    };

    if input.is_empty() {
        eprintln!("The file is empty");
        process::exit(1);
    }

    let lines: Vec<&str> = input.split("\n").collect();
    let mut subtitles = get::subs_from_lines(lines);

    let reset_numbers = cli.reset_numbers;

    match cli.command {
        Some(cli::Commands::Add { seconds }) => match seconds {
            Some(seconds) => {
                action::add_secs(&mut subtitles, seconds);
                let command = cli.command.unwrap();
                action::print_change_applied(command, seconds);

                if reset_numbers {
                    action::reset_nums(&mut subtitles);
                }
            }
            None => {
                eprintln!("Error: seconds no provided");
                process::exit(1);
            }
        },
        Some(cli::Commands::Subtract { seconds }) => match seconds {
            Some(seconds) => match action::sub_secs(&mut subtitles, seconds) {
                Ok(()) => {
                    let command = cli.command.unwrap();
                    action::print_change_applied(command, seconds);

                    if reset_numbers {
                        action::reset_nums(&mut subtitles);
                    }
                }
                Err(error) => {
                    eprintln!("Error: {}", error);
                    process::exit(1);
                }
            },
            None => {
                eprintln!("Error: seconds no provided");
                process::exit(1);
            }
        },
        None => {
            if reset_numbers {
                action::reset_nums(&mut subtitles);
            } else {
                eprintln!("No command or option provided");
                process::exit(1);
            }
        }
    }

    let contents = get::text_contents_subs(subtitles);

    let output_dir = cli.output_dir.as_deref().unwrap_or("output");
    let output_dir = path::Path::new(output_dir);

    let filename = filepath.split("/").last().unwrap();
    let output_file = cli.output_file.as_deref().unwrap_or(filename);
    let output_file = output_dir.join(output_file).with_extension("srt");

    if let Err(e) = fs::create_dir_all(output_dir) {
        if e.kind() == io::ErrorKind::PermissionDenied {
            eprintln!("Insufficient permissions to create output directory");
            process::exit(1);
        }

        eprintln!("Error trying to create output directory: {}", e);
        process::exit(1);
    }

    if let Err(e) = fs::write(&output_file, contents) {
        eprintln!("Error trying to write output file: {}", e);
        process::exit(1);
    }

    println!("Output file {} written", output_file.to_str().unwrap());

    Ok(())
}
