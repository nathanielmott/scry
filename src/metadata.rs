use chrono::prelude::*;
use eyre::ErrReport;
use std::fs::{canonicalize, metadata, Metadata};

#[cfg(not(target_os = "windows"))]
use std::os::unix::fs::MetadataExt as UnixMetadata;

#[derive(Debug)]
#[cfg(not(target_os = "windows"))]
pub struct FileData {
    pub ino: u64,
    pub user: String,
    pub path: String,
    pub size: u64,
    pub created: String,
    pub modified: String,
    pub accessed: String,
    pub blksize: u64,
    pub blocks: u64,
}
#[cfg(not(target_os = "windows"))]
impl FileData {
    pub fn new(file: &str) -> eyre::Result<FileData, ErrReport> {
        let data: Metadata = metadata(file)?;
        let ino = data.ino();
        let user = match users::get_user_by_uid(data.uid()) {
            Some(user) => user.name().to_str().unwrap().to_string(),
            None => data.uid().to_string(),
        };
        let path: String = match canonicalize(file) {
            Ok(path) => path
                .into_os_string()
                .into_string()
                .expect("could not resolve path"),
            Err(..) => String::from("could not resolve path"),
        };
        let size = data.len();
        let created: DateTime<Local> = DateTime::from(data.created()?);
        let modified: DateTime<Local> = DateTime::from(data.modified()?);
        let accessed: DateTime<Local> = DateTime::from(data.accessed()?);

        let formatted_created_time = format!("{}", created.format("%b %d, %Y %H:%M %Z"));
        let formatted_modified_time = format!("{}", modified.format("%b %d, %Y %H:%M %Z"));
        let formatted_accessed_time = format!("{}", accessed.format("%b %d, %Y %H:%M %Z"));

        let blksize = data.blksize();
        let blocks = data.blocks();

        Ok(FileData {
            ino,
            user,
            path,
            size,
            created: formatted_created_time,
            modified: formatted_modified_time,
            accessed: formatted_accessed_time,
            blksize,
            blocks,
        })
    }
}
#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt as WindowsMetadata;
#[cfg(target_os = "windows")]
use windows::Win32::System::Time;

#[derive(Debug)]
#[cfg(target_os = "windows")]
pub struct FileData {
    pub creation_time: String,
    pub last_access_time: String,
    pub last_write_time: String,
    pub file_size: u64,
}

#[cfg(target_os = "windows")]
impl FileData {
    pub fn new(file: &str) -> eyre::Result<FileData, ErrReport> {
        let metadata: Metadata = metadata(&file)?;

        // let
        let creation_time: DateTime<Utc> = DateTime::from(metadata.created()?);
        let last_access_time: DateTime<Utc> = DateTime::from(metadata.accessed()?);
        let last_write_time: DateTime<Utc> = DateTime::from(metadata.modified()?);

        let formatted_created_time = format!("{}", created.format("%b %d, %Y %H:%M %Z"));
        let formatted_modified_time = format!("{}", modified.format("%b %d, %Y %H:%M %Z"));
        let formatted_accessed_time = format!("{}", accessed.format("%b %d, %Y %H:%M %Z"));

        let file_size = metadata.file_size();

        Ok(FileData {
            creation_time: formatted_created_time,
            last_access_time: formatted_accessed_time,
            last_write_time: formatted_modified_time,
            file_size,
        })
    }
}
