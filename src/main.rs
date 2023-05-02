use clap::Parser;
use fmtsrt::{action, cli, get, util};
use std::{fs, io, io::prelude::*, path, process};

/// Maximum file size allowed to open in byte
const MAX_SIZE_FILE: u64 = 1_000_000;

fn main() -> io::Result<()> {
    let cli = cli::Cli::parse();

    let filepath = cli.input_file.as_deref().unwrap_or_else(|| {
        eprintln!("Use flag --input-file to provide the path of a SRT file");
        process::exit(1);
    });

    let mut file = match fs::File::open(filepath) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                eprintln!("File {} could not be found", filepath);
                process::exit(1);
            }
            io::ErrorKind::PermissionDenied => {
                eprintln!("Insufficient permissions to open the file {}", filepath);
                process::exit(1);
            }
            _ => {
                eprintln!("Could not open the file {}. {:?}", filepath, e);
                process::exit(1);
            }
        },
    };

    let size = file.metadata().map(|m| m.len()).unwrap_or(0);
    if size > MAX_SIZE_FILE {
        eprintln!(
            "Too large file, exceeds the limit of {}MB",
            util::bytes_to_megabytes(MAX_SIZE_FILE)
        );
        process::exit(1);
    }

    let mut input = String::with_capacity(size as usize);
    if let Err(e) = file.read_to_string(&mut input) {
        match e.kind() {
            io::ErrorKind::InvalidData => {
                eprintln!("Could not read the file {}, make sure you are passing a text file with valid UTF-8 content", filepath);
                process::exit(1);
            }
            _ => {
                eprintln!("Could not read the file {}. {:?}", filepath, e);
                process::exit(1);
            }
        }
    }

    if input.is_empty() {
        eprintln!("The file is empty");
        process::exit(1);
    }

    let lines: Vec<&str> = input.split('\n').collect();
    let mut subtitles = get::subs_from_lines(lines);

    match cli.command {
        cli::Commands::Add { seconds } => {
            action::add_secs(&mut subtitles, seconds);
            action::print_change_applied(cli.command, seconds);
        }
        cli::Commands::Sub { seconds } => {
            if let Err(e) = action::sub_secs(&mut subtitles, seconds) {
                eprintln!("{}", e);
                process::exit(1);
            }

            action::print_change_applied(cli.command, seconds);
        }
    }

    if cli.reset_numbers {
        action::reset_nums(&mut subtitles);
    }

    let contents = get::text_contents_subs(subtitles);

    let output_dir = cli.output_dir.as_deref().unwrap_or("output");
    let output_dir = path::Path::new(output_dir);

    let filename = filepath.split('/').last().unwrap();
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
