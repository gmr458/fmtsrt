use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "fmtsrt")]
#[command(version = "0.1.5")]
#[command(author = "German David <germanmarinrolong@gmail.com>")]
#[command(about = "CLI tool for basic SRT file edit", long_about = None)]
pub struct Cli {
    /// Path of the SRT file to edit
    #[arg(long)]
    pub input_file: Option<String>,

    /// Reset numbers of the SRT file
    #[arg(long)]
    pub reset_numbers: bool,

    /// Commands to add and subtract seconds in SRT file
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Directory to save the resulting file
    #[arg(long)]
    pub output_dir: Option<String>,

    /// Name of the resulting SRT file
    #[arg(long)]
    pub output_file: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add seconds to the .srt file, should be a positive integer
    Add { seconds: Option<u64> },

    /// Subtract seconds to the .srt file, should be a positive integer
    Subtract { seconds: Option<u64> },
}
