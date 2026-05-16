use crate::types::RequestEvent;
use anyhow::Result;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{fs::OpenOptions, io::AsyncWriteExt, sync::Mutex};
#[derive(Clone)]
pub struct JsonlLogger {
    path: Arc<PathBuf>,
    lock: Arc<Mutex<()>>,
}
impl JsonlLogger {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: Arc::new(path.as_ref().to_path_buf()),
            lock: Arc::new(Mutex::new(())),
        }
    }
    pub async fn append(&self, event: &RequestEvent) -> Result<()> {
        let _g = self.lock.lock().await;
        if let Some(p) = self.path.parent() {
            tokio::fs::create_dir_all(p).await?;
        }
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&*self.path)
            .await?;
        file.write_all(serde_json::to_string(event)?.as_bytes())
            .await?;
        file.write_all(b"\n").await?;
        Ok(())
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
}
