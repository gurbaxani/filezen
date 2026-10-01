use clap::Parser;
use colored::Colorize;
use filezen::organize_directory;
use std::path::PathBuf;

/// Command-line arguments for organizing your directory.
#[derive(Parser, Debug)]
#[command(
    name = "filezen",
    author,
    version,
    about = "Organize your cluttered directory by moving your files into categorized subfolders based on their extensions.",
    long_about = "FileZen tidies up your cluttered directories by categorizing and moving your files into dedicated subfolders (such as Images, Documents, Videos, Audio, Archives, Code, and Misc) based on their file extensions."
)]
struct Cli {
    /// The target directory you want to organize. Defaults to your current working directory.
    #[arg(default_value = ".")]
    path: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let target_dir = &cli.path;

    // Validate that your target path exists
    if !target_dir.exists() {
        eprintln!(
            "{} Your specified target directory '{}' does not exist.",
            "Error:".red().bold(),
            target_dir.display()
        );
        std::process::exit(1);
    }

    // Validate that your target path is actually a directory
    if !target_dir.is_dir() {
        eprintln!(
            "{} Your specified path '{}' is a file, not a directory.",
            "Error:".red().bold(),
            target_dir.display()
        );
        std::process::exit(1);
    }

    println!(
        "{} Organizing files in your directory: '{}'...",
        "FileZen:".cyan().bold(),
        target_dir.display()
    );

    // Run the core library logic (following rust-skills proj-lib-main-split)
    match organize_directory(target_dir) {
        Ok(summary) => {
            println!();
            if summary.moved_count > 0 {
                println!(
                    "{} You have organized {} file(s) into categorized subfolders!",
                    "Success:".green().bold(),
                    summary.moved_count
                );
            } else {
                println!(
                    "{} Your directory '{}' has no files that need organizing.",
                    "Info:".blue().bold(),
                    target_dir.display()
                );
            }

            if summary.warning_count > 0 {
                eprintln!(
                    "{} {} file(s) could not be moved. Please review the warnings above.",
                    "Notice:".yellow().bold(),
                    summary.warning_count
                );
            }
        }
        Err(err) => {
            eprintln!(
                "{} Could not open your target directory '{}': {}",
                "Error:".red().bold(),
                target_dir.display(),
                err
            );
            std::process::exit(1);
        }
    }
}
