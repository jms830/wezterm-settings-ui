// Backdrop commands - manage background images

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Information about a backdrop image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackdropImage {
    pub filename: String,
    pub path: String,
}

/// List all images in a directory that match common image formats
#[tauri::command]
pub fn list_backdrop_images(directory: String) -> Result<Vec<BackdropImage>, String> {
    let dir_path = PathBuf::from(&directory);
    
    if !dir_path.exists() {
        return Ok(Vec::new());
    }
    
    if !dir_path.is_dir() {
        return Err(format!("{} is not a directory", directory));
    }
    
    let image_extensions = ["jpg", "jpeg", "png", "gif", "bmp", "ico", "tiff", "pnm", "dds", "tga", "webp"];
    
    let mut images = Vec::new();
    
    match std::fs::read_dir(&dir_path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        let ext_lower = ext.to_string_lossy().to_lowercase();
                        if image_extensions.contains(&ext_lower.as_str()) {
                            images.push(BackdropImage {
                                filename: path.file_name()
                                    .map(|n| n.to_string_lossy().to_string())
                                    .unwrap_or_default(),
                                path: path.to_string_lossy().to_string(),
                            });
                        }
                    }
                }
            }
        }
        Err(e) => return Err(format!("Failed to read directory: {}", e)),
    }
    
    // Sort by filename
    images.sort_by(|a, b| a.filename.cmp(&b.filename));
    
    Ok(images)
}
