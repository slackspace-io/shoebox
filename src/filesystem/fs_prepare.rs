use anyhow::Result;
use chrono::{DateTime, Local};
use log::{info, warn};
use std::fs;
use std::path::Path;

pub fn copy_files_to_destination(file_paths: &Vec<String>, destination_root: String) -> Result<()> {
    let now: DateTime<Local> = Local::now();
    let dir_name = now.format("%Y-%m-%d_%H-%M-%S").to_string(); // Format the date and time
    let destination_path = Path::new(&destination_root).join(&dir_name); // Create unique directory

    // Create the destination directory
    fs::create_dir_all(&destination_path)?;
    info!(
        "Created destination directory: {}",
        destination_path.display()
    );

    // ... (rest of your copy logic remains the same, using destination_path)

    for file_path in file_paths {
        let source = Path::new(&file_path);
        let file_name = source.file_name().unwrap().to_str().unwrap(); // Get the file name
        let destination_file = destination_path.join(file_name);

        // Use the new path
        if destination_file.exists() {
            warn!(
                "File already exists, skipping: {}",
                destination_file.display()
            );
        } else {
            match fs::copy(source, &destination_file) {
                Ok(_) => info!("Copied {} to {}", file_path, destination_file.display()),
                Err(e) => return Err(anyhow::anyhow!("Failed to copy {}: {}", file_path, e)),
            }
        }
        // ... (your existing file copy and logging logic)
    }
    Ok(())
}
