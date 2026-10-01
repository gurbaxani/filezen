use clap::Parser;
use colored::Colorize;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

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

/// Categorizes a file path into a logical folder name without using regular expressions.
fn get_category(path: &Path) -> &'static str {
    // Check compound archive extensions first (e.g. .tar.gz, .tar.bz2, .tar.xz)
    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        let name_lower = file_name.to_lowercase();
        if name_lower.ends_with(".tar.gz")
            || name_lower.ends_with(".tar.bz2")
            || name_lower.ends_with(".tar.xz")
            || name_lower.ends_with(".tar.zst")
        {
            return "Archives";
        }
    }

    // Standard extension extraction using std::path::Path::extension
    let ext = match path.extension().and_then(|e| e.to_str()) {
        Some(e) => e.to_lowercase(),
        None => return "Misc",
    };

    match ext.as_str() {
        // Images
        "jpg" | "jpeg" | "png" | "webp" | "gif" | "bmp" | "svg" | "ico" | "tiff" | "tif"
        | "heic" | "avif" | "raw" => "Images",

        // Documents
        "pdf" | "docx" | "doc" | "txt" | "rtf" | "odt" | "xlsx" | "xls" | "ods" | "pptx"
        | "ppt" | "odp" | "csv" | "tsv" | "md" => "Documents",

        // Videos
        "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" | "m4v" | "mpg" | "mpeg"
        | "3gp" => "Videos",

        // Audio
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" | "wma" | "opus" | "alac" | "aiff" => {
            "Audio"
        }

        // Archives
        "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" | "zst" | "tgz" | "tbz2" | "iso" => {
            "Archives"
        }

        // Code & Scripts
        "rs" | "py" | "js" | "ts" | "jsx" | "tsx" | "html" | "htm" | "css" | "scss" | "json"
        | "xml" | "yaml" | "yml" | "toml" | "c" | "cpp" | "h" | "hpp" | "cs" | "go" | "java"
        | "kt" | "swift" | "php" | "rb" | "sh" | "bash" | "zsh" | "sql" => "Code",

        // Unrecognized extensions go to Misc
        _ => "Misc",
    }
}

/// Generates a non-colliding destination path if a file with the same name already exists.
fn get_unique_destination(target_path: PathBuf) -> PathBuf {
    if !target_path.exists() {
        return target_path;
    }

    let parent = target_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();

    let file_stem = target_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("file")
        .to_string();

    // Check if the original name had a compound extension like .tar.gz
    let (stem, ext_suffix) =
        if let Some(file_name) = target_path.file_name().and_then(|s| s.to_str()) {
            let lower = file_name.to_lowercase();
            if lower.ends_with(".tar.gz") {
                let stem_part = &file_name[..file_name.len() - ".tar.gz".len()];
                (stem_part.to_string(), ".tar.gz".to_string())
            } else if lower.ends_with(".tar.bz2") {
                let stem_part = &file_name[..file_name.len() - ".tar.bz2".len()];
                (stem_part.to_string(), ".tar.bz2".to_string())
            } else if lower.ends_with(".tar.xz") {
                let stem_part = &file_name[..file_name.len() - ".tar.xz".len()];
                (stem_part.to_string(), ".tar.xz".to_string())
            } else {
                let ext = target_path
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|e| format!(".{}", e))
                    .unwrap_or_default();
                (file_stem, ext)
            }
        } else {
            let ext = target_path
                .extension()
                .and_then(|s| s.to_str())
                .map(|e| format!(".{}", e))
                .unwrap_or_default();
            (file_stem, ext)
        };

    let mut counter = 1;
    loop {
        let candidate_name = format!("{}_{}{}", stem, counter, ext_suffix);
        let candidate_path = parent.join(&candidate_name);
        if !candidate_path.exists() {
            return candidate_path;
        }
        counter += 1;
    }
}

/// Moves a file from `src` to `dst`, with fallback for cross-device moves.
fn move_file(src: &Path, dst: &Path) -> io::Result<()> {
    if let Err(err) = fs::rename(src, dst) {
        // If files are on different filesystems/devices, fall back to copy then remove
        if err.kind() == io::ErrorKind::CrossesDevices {
            fs::copy(src, dst)?;
            fs::remove_file(src)?;
        } else {
            return Err(err);
        }
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    let target_dir = &cli.path;

    // Validate that the target path exists
    if !target_dir.exists() {
        eprintln!(
            "{} Your specified target directory '{}' does not exist.",
            "Error:".red().bold(),
            target_dir.display()
        );
        std::process::exit(1);
    }

    // Validate that the target path is actually a directory
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

    // Read the contents of the target directory
    let read_dir = match fs::read_dir(target_dir) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!(
                "{} Could not open your target directory '{}': {}",
                "Error:".red().bold(),
                target_dir.display(),
                err
            );
            std::process::exit(1);
        }
    };

    let mut moved_count: usize = 0;
    let mut warning_count: usize = 0;

    for entry_result in read_dir {
        // Non-panicking entry handling
        let entry = match entry_result {
            Ok(e) => e,
            Err(err) => {
                eprintln!(
                    "{} Could not read a directory entry in your target directory '{}': {}",
                    "Warning:".yellow().bold(),
                    target_dir.display(),
                    err
                );
                warning_count += 1;
                continue;
            }
        };

        let file_path = entry.path();

        // Query file type without following symlinks first
        let file_type = match entry.file_type() {
            Ok(ft) => ft,
            Err(err) => {
                eprintln!(
                    "{} Could not determine file type for your item '{}': {}",
                    "Warning:".yellow().bold(),
                    file_path.display(),
                    err
                );
                warning_count += 1;
                continue;
            }
        };

        // Skip directories: Ensure the tool only attempts to move files, ignoring nested directories
        let is_directory = if file_type.is_dir() {
            true
        } else if file_type.is_symlink() {
            // Check if the symlink resolves to a directory
            match fs::metadata(&file_path) {
                Ok(meta) => meta.is_dir(),
                Err(_) => false,
            }
        } else {
            false
        };

        if is_directory {
            continue;
        }

        // Get the file name
        let file_name = match file_path.file_name() {
            Some(name) => name.to_string_lossy().into_owned(),
            None => continue,
        };

        // Categorize file based on extension
        let category = get_category(&file_path);
        let category_dir = target_dir.join(category);

        // Check if the target subdirectory exists, create it if it doesn't
        if !category_dir.exists() {
            if let Err(err) = fs::create_dir_all(&category_dir) {
                eprintln!(
                    "{} Could not create category folder '{}' for your file '{}': {}",
                    "Warning:".yellow().bold(),
                    category_dir.display(),
                    file_path.display(),
                    err
                );
                warning_count += 1;
                continue;
            }
        }

        // Determine destination path and handle possible filename collisions safely
        let default_destination = category_dir.join(&file_name);
        let final_destination = get_unique_destination(default_destination.clone());
        let was_renamed = final_destination != default_destination;

        // Move the file with non-panicking error handling
        match move_file(&file_path, &final_destination) {
            Ok(()) => {
                moved_count += 1;
                if was_renamed {
                    let new_file_name = final_destination
                        .file_name()
                        .map(|n| n.to_string_lossy())
                        .unwrap_or_default();
                    println!(
                        "  {} Moved '{}' -> '{}/{}' (renamed to protect your existing file)",
                        "✔".green().bold(),
                        file_name,
                        category,
                        new_file_name
                    );
                } else {
                    println!(
                        "  {} Moved '{}' -> '{}/'",
                        "✔".green().bold(),
                        file_name,
                        category
                    );
                }
            }
            Err(err) => {
                eprintln!(
                    "{} Could not move your file '{}' to '{}': {}",
                    "Warning:".yellow().bold(),
                    file_path.display(),
                    final_destination.display(),
                    err
                );
                warning_count += 1;
            }
        }
    }

    // Print clear summary for the user
    println!();
    if moved_count > 0 {
        println!(
            "{} You have organized {} file(s) into categorized subfolders!",
            "Success:".green().bold(),
            moved_count
        );
    } else {
        println!(
            "{} Your directory '{}' has no files that need organizing.",
            "Info:".blue().bold(),
            target_dir.display()
        );
    }

    if warning_count > 0 {
        eprintln!(
            "{} {} file(s) could not be moved. Please review the warnings above.",
            "Notice:".yellow().bold(),
            warning_count
        );
    }
}
