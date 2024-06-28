use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use tokio::fs::{self, File};
use tokio::io::Result;

pub async fn get_file(filepath: PathBuf) -> Result<File> {
    match filepath.extension() {
        None => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Filepath provided have no extension.",
            ))
        }
        Some(_) => (),
    };

    if !filepath.parent().unwrap().exists() {
        fs::create_dir_all(&filepath.parent().unwrap())
            .await
            .unwrap();
    }

    let file = if !filepath.exists() {
        File::create(&filepath).await.unwrap()
    } else {
        File::open(&filepath).await.unwrap()
    };

    Ok(file)
}
