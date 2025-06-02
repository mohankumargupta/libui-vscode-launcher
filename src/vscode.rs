use std::{fs, path::PathBuf};

use directories::UserDirs;

pub fn downloads_dir() -> Option<PathBuf>{
    UserDirs::new().and_then(|dirs|{
        dirs.download_dir().map(|dir| dir.to_path_buf())
    })
}

pub fn get_vscode_portable_folder_names() -> Result<Vec<String>, std::io::Error> {
    let mut vscode_folders = Vec::new();

    if let Some(user_dirs) = UserDirs::new() {
        if let Some(downloads_dir) = user_dirs.download_dir() { // [2, 5]
            println!("Searching in Downloads directory: {:?}", downloads_dir); // For debugging

            match fs::read_dir(downloads_dir) { // [3, 4, 8]
                Ok(entries) => {
                    for entry in entries {
                        match entry {
                            Ok(dir_entry) => {
                                if dir_entry.file_type()?.is_dir() {
                                    if let Some(folder_name) = dir_entry.file_name().to_str() {
                                        if folder_name.starts_with("vscode-") {
                                            vscode_folders.push(folder_name.to_string());
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Warning: Could not read directory entry: {}", e);
                                // Decide if you want to propagate this error or just skip the entry
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading Downloads directory: {}", e);
                    return Err(e);
                }
            }
        } else {
            eprintln!("Could not find Downloads directory.");
            // You might want to return a specific error type here
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Downloads directory not found"));
        }
    } else {
        eprintln!("Could not retrieve user directories.");
        // You might want to return a specific error type here
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "User directories not found"));
    }

    Ok(vscode_folders)
}

