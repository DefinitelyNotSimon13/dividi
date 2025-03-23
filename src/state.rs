use std::collections::HashSet;

use tokio::{fs, sync::Mutex};
use tracing::{error, info};

use crate::uploaded_file::{UPLOAD_DIR, UploadedFile};

#[derive(Debug)]
pub struct AppState {
    files: Mutex<HashSet<UploadedFile>>,
}

impl AppState {
    pub async fn new() -> Self {
        let files = Self::load_existing_files().await;
        Self {
            files: Mutex::new(files.into_iter().collect()),
        }
    }

    async fn load_existing_files() -> Vec<UploadedFile> {
        let mut uploaded_files = Vec::new();

        if let Err(e) = fs::create_dir_all(UPLOAD_DIR).await {
            error!("Failed to create uploads directory '{}': {}", UPLOAD_DIR, e);
            return uploaded_files;
        }

        match fs::read_dir(UPLOAD_DIR).await {
            Ok(mut dir) => {
                while let Ok(Some(entry)) = dir.next_entry().await {
                    let path = entry.path();
                    if let Some(file) = UploadedFile::from_path(path).await {
                        uploaded_files.push(file);
                    }
                }
            }
            Err(e) => {
                error!("Failed to read uploads directory '{}': {}", UPLOAD_DIR, e);
            }
        }
        info!(
            "Loaded {} files from '{}'",
            uploaded_files.len(),
            UPLOAD_DIR
        );
        uploaded_files
    }

    pub async fn upload_file(&self, file: UploadedFile) {
        let mut files = self.files.lock().await;
        files.insert(file);
    }

    pub async fn remove_file(&self, file_name: &str) {
        self.files
            .lock()
            .await
            .retain(|file| file.name != file_name);
    }

    pub async fn files(&self) -> HashSet<UploadedFile> {
        self.files.lock().await.clone()
    }

    pub async fn files_as_vec(&self) -> Vec<UploadedFile> {
        self.files().await.into_iter().collect()
    }
}
