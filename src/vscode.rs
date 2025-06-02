use std::{
    fs,
    io,
    path::{Path, PathBuf},
};
use directories::UserDirs;

// Helper to create a standard "Not Found" IO error
fn io_error_not_found(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::NotFound, message)
}

pub fn downloads_dir() -> Option<PathBuf> {
    UserDirs::new().and_then(|dirs| dirs.download_dir().map(Path::to_path_buf))
}

pub fn get_vscode_portable_folder_names() -> Result<Vec<String>, io::Error> {
    let downloads_path = downloads_dir()
        .ok_or_else(|| io_error_not_found("Downloads directory not found via UserDirs."))?;

    // For debugging, you might want this behind a feature flag or config
    // println!("Searching in Downloads directory: {:?}", downloads_path);

    let entries = fs::read_dir(downloads_path)?; // Propagates error from reading directory itself

    let vscode_folders = entries
        .filter_map(|entry_result| {
            match entry_result {
                Ok(entry) => {
                    let path = entry.path();
                    if path.is_dir() {
                        // Check if file_name is valid UTF-8 and starts with "vscode-"
                        path.file_name()
                            .and_then(|os_name| os_name.to_str()) // Option<&str>
                            .filter(|name| name.starts_with("vscode-")) // Option<&str>
                            .map(String::from) // Option<String>
                    } else {
                        None // Not a directory
                    }
                }
                Err(e) => {
                    // Log individual entry errors but don't stop processing the whole directory
                    eprintln!(
                        "Warning: Failed to process a directory entry: {}. Skipping.",
                        e
                    );
                    None // Skip this problematic entry
                }
            }
        })
        .collect(); // Collects Vec<String>

    Ok(vscode_folders)
}