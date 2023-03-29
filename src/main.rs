use clap::{Parser, Subcommand};
use fmtsrt::{action, get};
use std::{fs, io, path, process};

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let filepath = cli.input_file.as_deref().unwrap_or_else(|| {
        eprintln!("the path of the .srt file should be provided");
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
            // io::ErrorKind::IsADirectory => {
            //     eprintln!("You passed a directory, make sure it is a text file");
            //     process::exit(1);
            // }
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
        Some(Commands::Add { seconds }) => match seconds {
            Some(seconds) => {
                action::add_secs(&mut subtitles, seconds);

                if reset_numbers {
                    action::reset_nums(&mut subtitles);
                }
            }
            None => {
                eprintln!("Error: seconds no provided");
                process::exit(1);
            }
        },
        Some(Commands::Subtract { seconds }) => match seconds {
            Some(seconds) => match action::sub_secs(&mut subtitles, seconds) {
                Ok(()) => {
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

    match fs::create_dir_all(output_dir) {
        Ok(()) => {
            println!("Directory {} created", output_dir.to_str().unwrap());
        }
        Err(error) => match error.kind() {
            io::ErrorKind::PermissionDenied => {
                eprintln!("Insufficient permissions to read the file");
                process::exit(1);
            }
            _ => {
                eprintln!(
                    "Unexpected error trying to create de output directory: {}",
                    error
                );
                process::exit(1);
            }
        },
    };
    match fs::write(output_file, contents) {
        Ok(()) => {
            println!("Output file .srt write written");
        }
        Err(error) => {
            eprintln!(
                "Unexpected error trying to create de output directory: {}",
                error
            );
            process::exit(1);
        }
    };

    Ok(())
}

#[derive(Parser)]
#[command(author, version, about = "CLI tool for basic SRT file edit", long_about = None)]
struct Cli {
    /// Path of the .srt file
    #[arg(long)]
    input_file: Option<String>,

    /// Reset numbers of the .srt file
    #[arg(long)]
    reset_numbers: bool,

    /// Commands to add and subtract seconds in .srt file
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path of the directory to save the resulting file
    #[arg(long)]
    output_dir: Option<String>,

    /// Name of the resulting file without .srt extension
    #[arg(long)]
    output_file: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add seconds to the .srt file, should be a positive integer
    Add { seconds: Option<u64> },

    /// Subtract seconds to the .srt file, should be a positive integer
    Subtract { seconds: Option<u64> },
}
