use std::env;


use super::error::TgitError;
use super::file::FileService;
use super::index::Index;
use super::types::Blob;

pub fn add_all(add_data: &Vec<&str>) -> Result<(), TgitError> {
    let file_service = FileService::new()?;
    let current_dir = env::current_dir()?;
    let mut index = Index::new(&file_service.root_dir)?;

    for file in add_data {
        let full_path = current_dir.join(file);
        let blob = Blob::from_path(&full_path)?;
        file_service.write_blob(&blob)?;
        let relative_path = full_path.strip_prefix(&file_service.root_dir).unwrap();
        index.update(&relative_path.to_str().unwrap(), &blob.hash)
    }
    index.write()?;
    Ok(())
}