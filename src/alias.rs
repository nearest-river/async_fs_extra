
use tokio::fs;
use std::{
  io::Result,
  path::Path,
};

pub async fn exists<P: AsRef<Path>>(path: P)-> bool {
  fs::try_exists(path).await
  .unwrap_or(false)
}

pub async fn is_dir<P: AsRef<Path>>(path: P)-> bool {
  fs::metadata(path).await
  .map(|m| m.is_dir())
  .unwrap_or(false)
}

pub async fn is_file<P: AsRef<Path>>(path: P)-> bool {
  fs::metadata(path).await
  .map(|m| m.is_file())
  .unwrap_or(false)
}


