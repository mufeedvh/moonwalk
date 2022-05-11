use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::io::Result;
use std::process::Command;

use super::parsers::nix_stat_parser;

use serde::{Deserialize, Serialize};

pub struct FileSystem;

#[derive(Serialize, Deserialize)]
pub struct FileStat {
    pub atime: String,
    pub mtime: String,
    pub ctime: String
}

impl FileSystem {
    /// Returns stat info of files to parse access/modify timestamps
    pub fn file_nix_stat(file_path: &str) -> FileStat {
        // return file stats from child process
        let child_process = Command::new("/bin/stat")
            .arg(file_path)
            .output()
            .expect("failed to execute child process");

        // parse unix timestamp from fs stats
        nix_stat_parser(
            String::from_utf8_lossy(&child_process.stdout)
        )
    }

    /// Apply timestamps to files using the touch utility
    #[inline]
    pub fn change_file_timestamp(file_path: &str, stat: FileStat) {
        Command::new("/usr/bin/touch")
            .args([
                "-a", "-t", &stat.atime,
                "-m", "-t", &stat.mtime,
                file_path
            ])
            .output()
            .expect("failed to execute child process");
    }

    /// Returns if a file path exists or not
    #[inline]
    pub fn file_exists(file_path: &str) -> bool {
        Path::new(file_path).exists()
    }

    /// Read a file into bytes
    pub fn read(file_path: &str) -> Result<Vec<u8>> {
        let file = fs::File::open(file_path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents: Vec<u8> = Vec::new();
        buf_reader.read_to_end(&mut contents)?;
        Ok(contents)
    }

    /// Write bytes to a file
    pub fn write(file_path: &str, contents: &[u8]) -> Result<()> {
        let mut file = fs::File::create(file_path)?;
        file.write_all(contents)?;
        Ok(())        
    }

    /// Create a recursive directory
    pub fn create_dir(file_path: &str) -> Result<()> {
        if !Path::new(file_path).exists() {
            fs::create_dir_all(file_path)?
        }
        Ok(())
    }

    /// Remove a directory at absolute path
    pub fn remove_dir(file_path: &str) -> Result<()> {
        if Path::new(file_path).exists() {
            fs::remove_dir_all(file_path)?
        }
        Ok(())
    }
}