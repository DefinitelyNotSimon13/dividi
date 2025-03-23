use std::path::{Path, PathBuf};

use bytes::Bytes;
use humansize::format_size;
use sanitize_filename::sanitize;
use tokio::fs;
use tracing::{error, info};

pub static UPLOAD_DIR: &str = "uploads";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct UploadedFile {
    pub name: String,
    pub path: PathBuf,
    pub size: usize,
}

impl UploadedFile {
    pub async fn save(
        custom_file_name: Option<String>,
        orig_file_name: Option<String>,
        file_data: Bytes,
    ) -> anyhow::Result<Self> {
        // Determine the final file name.
        let file_name = if let Some(user_title) = custom_file_name {
            let clean_title = sanitize(&user_title);
            if let Some(orig) = orig_file_name.as_ref() {
                if let Some(ext) = Path::new(orig).extension().and_then(|s| s.to_str()) {
                    format!("{}.{}", clean_title, ext)
                } else {
                    clean_title
                }
            } else {
                clean_title
            }
        } else {
            orig_file_name.unwrap_or_else(|| "unknown_title".to_string())
        };

        // Ensure the upload directory exists.
        fs::create_dir_all(UPLOAD_DIR).await?;
        let file_path_str = format!("{}/{}", UPLOAD_DIR, file_name);
        let file_path = PathBuf::from(&file_path_str);
        // Write the file data.
        fs::write(&file_path, &file_data).await?;
        // Retrieve metadata for file size.
        let metadata = fs::metadata(&file_path).await?;
        let size = metadata.len() as usize;

        info!("sile saved successfully as {}", file_path_str);

        Ok(UploadedFile {
            name: file_name,
            path: file_path,
            size,
        })
    }

    pub async fn from_path(path: PathBuf) -> Option<Self> {
        match fs::metadata(&path).await {
            Ok(metadata) if metadata.is_file() => {
                let size = metadata.len() as usize;
                if let Some(name_os) = path.file_name() {
                    if let Some(name) = name_os.to_str() {
                        info!("loaded file: {} ({} bytes)", name, size);
                        return Some(UploadedFile {
                            name: name.to_string(),
                            path,
                            size,
                        });
                    }
                }
                None
            }
            Ok(_) => None, // Skip if not a file.
            Err(e) => {
                error!("failed to get metadata for {:?}: {}", path, e);
                None
            }
        }
    }

    pub fn human_readable_size(&self) -> String {
        format_size(self.size as u64, humansize::DECIMAL)
    }
}
