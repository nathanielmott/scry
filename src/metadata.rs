use chrono::prelude::*;
use eyre::ErrReport;
use std::fs::{metadata, Metadata};
use std::os::unix::fs::MetadataExt as UnixMetadata;
// use std::os::windows::fs::MetadataExt as WindowsMetadata;

#[derive(Debug)]
#[cfg(not(target_os = "windows"))]
pub struct FileData {
    pub ino: u64,
    pub uid: u32,
    pub size: u64,
    pub created: String,
    pub modified: String,
    pub accessed: String,
    pub blksize: u64,
    pub blocks: u64,
}

#[cfg(not(target_os = "windows"))]
pub fn get_data(file: &str) -> eyre::Result<FileData, ErrReport> {
    let metadata: Metadata = metadata(&file)?;
    let ino = metadata.ino();
    let uid = metadata.uid();
    let size = metadata.len();
    let created: DateTime<Local> = DateTime::from(metadata.created()?);
    let modified: DateTime<Local> = DateTime::from(metadata.modified()?);
    let accessed: DateTime<Local> = DateTime::from(metadata.accessed()?);

    let formatted_created_time = format!("{}", created.format("%m/%d/%Y %H:%M"));
    let formatted_modified_time = format!("{}", modified.format("%m/%d/%Y %H:%M"));
    let formatted_accessed_time = format!("{}", accessed.format("%m/%d/%Y %H:%M"));

    let blksize = metadata.blksize();
    let blocks = metadata.blocks();

    Ok(FileData {
        ino,
        uid,
        size,
        created: formatted_created_time,
        modified: formatted_modified_time,
        accessed: formatted_accessed_time,
        blksize,
        blocks,
    })
}

#[derive(Debug)]
#[cfg(target_os = "windows")]
pub struct FileData {
    pub creation_time: String,
    pub last_access_time: String,
    pub last_write_time: String,
    pub file_size: u64,
}
#[cfg(target_os = "windows")]
pub fn get_data(file: &str) -> eyre::Result<FileData, ErrReport> {
    let metadata: Metadata = metadata(&file)?;
    let creation_time: DateTime<Utc> = DateTime::from(metadata.created()?);
    let last_access_time: DateTime<Utc> = DateTime::from(metadata.accessed()?);
    let last_write_time: DateTime<Utc> = DateTime::from(metadata.modified()?);

    let formatted_created_time = format!("{}", creation_time.format("%m/%d/%Y %H:%M"));
    let formatted_accessed_time = format!("{}", last_access_time.format("%m/%d/%Y %H:%M"));
    let formatted_modified_time = format!("{}", last_write_time.format("%m/%d/%Y %H:%M"));

    let file_size = metadata.file_size();

    Ok(FileData {
        creation_time: formatted_created_time,
        last_access_time: formatted_accessed_time,
        last_write_time: formatted_modified_time,
        file_size,
    })
}
