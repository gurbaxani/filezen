use colored::Colorize;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Summary of an organization run.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Summary {
    pub moved_count: usize,
    pub warning_count: usize,
}

/// Logical category for organized files.
///
/// Implements `type-no-stringly` by replacing magic strings with a strongly-typed,
/// exhaustive enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    Images,
    Documents,
    Videos,
    Audio,
    Archives,
    Code,
    Misc,
}

impl Category {
    /// Returns the static directory name corresponding to this category.
    #[inline]
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Images => "Images",
            Self::Documents => "Documents",
            Self::Videos => "Videos",
            Self::Audio => "Audio",
            Self::Archives => "Archives",
            Self::Code => "Code",
            Self::Misc => "Misc",
        }
    }

    /// Identifies the category for a given path based on its filename and extension.
    ///
    /// Follows `name-no-get-prefix` by using idiomatic constructor naming `from_path`.
    #[must_use]
    pub fn from_path(path: &Path) -> Self {
        // Check compound archive extensions first (e.g., .tar.gz, .tar.bz2, .tar.xz)
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            let name_lower = file_name.to_lowercase();
            if name_lower.ends_with(".tar.gz")
                || name_lower.ends_with(".tar.bz2")
                || name_lower.ends_with(".tar.xz")
                || name_lower.ends_with(".tar.zst")
            {
                return Self::Archives;
            }
        }

        // Standard extension extraction using std::path::Path::extension
        let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
            return Self::Misc;
        };

        let ext_lower = ext.to_lowercase();
        match ext_lower.as_str() {
            // Images
            "jpg" | "jpeg" | "png" | "webp" | "gif" | "bmp" | "svg" | "ico" | "tiff" | "tif"
            | "heic" | "avif" | "raw" => Self::Images,

            // Documents
            "pdf" | "docx" | "doc" | "txt" | "rtf" | "odt" | "xlsx" | "xls" | "ods" | "pptx"
            | "ppt" | "odp" | "csv" | "tsv" | "md" => Self::Documents,

            // Videos
            "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" | "m4v" | "mpg" | "mpeg"
            | "3gp" => Self::Videos,

            // Audio
            "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" | "wma" | "opus" | "alac"
            | "aiff" => Self::Audio,

            // Archives
            "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" | "zst" | "tgz" | "tbz2"
            | "iso" => Self::Archives,

            // Code & Scripts
            "rs" | "py" | "js" | "ts" | "jsx" | "tsx" | "html" | "htm" | "css" | "scss"
            | "json" | "xml" | "yaml" | "yml" | "toml" | "c" | "cpp" | "h" | "hpp" | "cs"
            | "go" | "java" | "kt" | "swift" | "php" | "rb" | "sh" | "bash" | "zsh" | "sql" => {
                Self::Code
            }

            // Unrecognized extensions
            _ => Self::Misc,
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Generates a non-colliding destination path if a file with the same name already exists.
///
/// Follows `name-no-get-prefix` (renamed from `get_unique_destination`),
/// `pat-let-else` for concise pattern binding, and avoids unnecessary allocations.
#[must_use]
pub fn unique_destination(target_path: PathBuf) -> PathBuf {
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
        .unwrap_or("file");

    // Check if the original name had a compound extension like .tar.gz
    let (stem, ext_suffix) = if let Some(file_name) = target_path.file_name().and_then(|s| s.to_str())
    {
        let lower = file_name.to_lowercase();
        if lower.ends_with(".tar.gz") {
            (&file_name[..file_name.len() - ".tar.gz".len()], ".tar.gz")
        } else if lower.ends_with(".tar.bz2") {
            (&file_name[..file_name.len() - ".tar.bz2".len()], ".tar.bz2")
        } else if lower.ends_with(".tar.xz") {
            (&file_name[..file_name.len() - ".tar.xz".len()], ".tar.xz")
        } else {
            let ext = target_path
                .extension()
                .and_then(|s| s.to_str())
                .map(|_| &file_name[file_stem.len()..])
                .unwrap_or("");
            (file_stem, ext)
        }
    } else {
        (file_stem, "")
    };

    let mut counter = 1usize;
    loop {
        let candidate_name = format!("{stem}_{counter}{ext_suffix}");
        let candidate_path = parent.join(&candidate_name);
        if !candidate_path.exists() {
            return candidate_path;
        }
        counter += 1;
    }
}

/// Moves a file from `src` to `dst`, with automatic fallback for cross-device moves.
///
/// # Errors
///
/// Returns an `io::Error` if the file cannot be moved, copied, or deleted due to
/// permission errors or filesystem locks.
pub fn move_file(src: &Path, dst: &Path) -> io::Result<()> {
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

/// Organizes all loose files in `target_dir` into categorized subdirectories.
///
/// # Errors
///
/// Returns an error if the directory cannot be opened.
pub fn organize_directory(target_dir: &Path) -> io::Result<Summary> {
    let read_dir = fs::read_dir(target_dir)?;

    let mut summary = Summary::default();

    for entry_result in read_dir {
        // Non-panicking entry handling using pat-let-else
        let Ok(entry) = entry_result else {
            if let Err(err) = entry_result {
                eprintln!(
                    "{} Could not read a directory entry in your target directory '{}': {}",
                    "Warning:".yellow().bold(),
                    target_dir.display(),
                    err
                );
            }
            summary.warning_count += 1;
            continue;
        };

        let file_path = entry.path();

        // Query file type without following symlinks first (pat-let-else)
        let Ok(file_type) = entry.file_type() else {
            if let Err(err) = entry.file_type() {
                eprintln!(
                    "{} Could not determine file type for your item '{}': {}",
                    "Warning:".yellow().bold(),
                    file_path.display(),
                    err
                );
            }
            summary.warning_count += 1;
            continue;
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

        // Get the file name using pat-let-else
        let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };

        // Categorize file based on extension using strongly-typed Category enum
        let category = Category::from_path(&file_path);
        let category_dir = target_dir.join(category.as_str());

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
                summary.warning_count += 1;
                continue;
            }
        }

        // Determine destination path and handle possible filename collisions safely
        let default_destination = category_dir.join(file_name);
        let final_destination = unique_destination(default_destination);
        let was_renamed = final_destination
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|name| name != file_name);

        // Move the file with non-panicking error handling
        match move_file(&file_path, &final_destination) {
            Ok(()) => {
                summary.moved_count += 1;
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
                summary.warning_count += 1;
            }
        }
    }

    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categorize_images() {
        assert_eq!(Category::from_path(Path::new("pic.jpg")), Category::Images);
        assert_eq!(Category::from_path(Path::new("PIC.PNG")), Category::Images);
        assert_eq!(Category::from_path(Path::new("logo.webp")), Category::Images);
        assert_eq!(Category::from_path(Path::new("icon.svg")), Category::Images);
    }

    #[test]
    fn test_categorize_documents() {
        assert_eq!(Category::from_path(Path::new("doc.pdf")), Category::Documents);
        assert_eq!(Category::from_path(Path::new("notes.txt")), Category::Documents);
        assert_eq!(Category::from_path(Path::new("sheet.xlsx")), Category::Documents);
        assert_eq!(Category::from_path(Path::new("readme.md")), Category::Documents);
    }

    #[test]
    fn test_categorize_videos() {
        assert_eq!(Category::from_path(Path::new("clip.mp4")), Category::Videos);
        assert_eq!(Category::from_path(Path::new("movie.mkv")), Category::Videos);
    }

    #[test]
    fn test_categorize_audio() {
        assert_eq!(Category::from_path(Path::new("song.mp3")), Category::Audio);
        assert_eq!(Category::from_path(Path::new("track.flac")), Category::Audio);
    }

    #[test]
    fn test_categorize_compound_archives() {
        assert_eq!(Category::from_path(Path::new("archive.tar.gz")), Category::Archives);
        assert_eq!(Category::from_path(Path::new("backup.tar.bz2")), Category::Archives);
        assert_eq!(Category::from_path(Path::new("dist.tar.xz")), Category::Archives);
        assert_eq!(Category::from_path(Path::new("plain.zip")), Category::Archives);
        assert_eq!(Category::from_path(Path::new("data.tar")), Category::Archives);
    }

    #[test]
    fn test_categorize_code() {
        assert_eq!(Category::from_path(Path::new("main.rs")), Category::Code);
        assert_eq!(Category::from_path(Path::new("script.py")), Category::Code);
        assert_eq!(Category::from_path(Path::new("index.ts")), Category::Code);
    }

    #[test]
    fn test_categorize_misc_and_unrecognized() {
        assert_eq!(Category::from_path(Path::new("unknown.xyz123")), Category::Misc);
        assert_eq!(Category::from_path(Path::new("LICENSE")), Category::Misc);
        assert_eq!(Category::from_path(Path::new(".gitignore")), Category::Misc);
    }

    #[test]
    fn test_unique_destination_no_collision() {
        let path = PathBuf::from("non_existent_unique_file_xyz.txt");
        assert_eq!(unique_destination(path.clone()), path);
    }

    #[test]
    fn test_unique_destination_with_collision() {
        let temp_dir = std::env::temp_dir().join("filezen_test_collision_lib");
        let _ = fs::create_dir_all(&temp_dir);
        let orig_file = temp_dir.join("test_item.txt");
        fs::write(&orig_file, b"content").unwrap();

        let resolved = unique_destination(orig_file.clone());
        assert_eq!(resolved, temp_dir.join("test_item_1.txt"));

        let _ = fs::remove_file(&orig_file);
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
