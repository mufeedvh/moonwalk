use std::io::Result;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use once_cell::sync::Lazy;

use super::{
    values,
    fs::{FileSystem, FileStat},
    recon::return_wr_dir,
    messages::{Type, push_message}
};

// set the logging directory inside a world writable directory in the target machine
pub static TMP_LOG_DIR: Lazy<String> = Lazy::new(|| {
    format!("{}/.MOONWALK", return_wr_dir())
});

pub(crate) struct Logger;

#[derive(Serialize, Deserialize)]
struct StatMap {
    log_files: Map<String, Value>
}

impl Logger {
    /// Prepares logging directory in a world-writable directory
    pub fn init() -> Result<()> {
        push_message(
            Type::Info,
            &format!("Found `{}` as world writable.", *TMP_LOG_DIR)
        );
        push_message(
            Type::Info,
            &format!("Set `{}` as the logging directory", *TMP_LOG_DIR)
        );
        FileSystem::create_dir(&TMP_LOG_DIR)?;
        Self::save_state()?;
        Ok(())
    }

    /// Saves the current state of log files including it's timestamps
    pub fn save_state() -> Result<()> {
        // get current authenticated user
        let user = &values::CURR_USER;

        // initiate a HashMap for saving the current timestamps of log files
        let mut file_stat_map = StatMap {
            log_files: Map::with_capacity(values::LOG_FILES.len())
        };

        // save states of all log files
        for log_file in values::LOG_FILES {
            let mut log_file: String = String::from(log_file);

            // parse and resolve `~/` home path
            if log_file.starts_with('~') {
                let current_user = format!(
                    "/home/{:?}/",
                    user.name()
                ).replace('"', "");

                log_file = log_file.replace("~/", &current_user);
            }

            if FileSystem::file_exists(&log_file) {
                // handle exact fs structure creation under logger directory
                let mut path: Vec<&str> = log_file.split('/').collect();
                path.pop();
                let dir_structure = path.join("/");

                // create the same directory structure moonwalk's tmp dir
                FileSystem::create_dir(
                    &format!("{}{}", *TMP_LOG_DIR, dir_structure)
                )?;

                // save target directory path
                let save_state_file = format!("{}{}", *TMP_LOG_DIR, log_file);

                // serialize the log file's stat timestamps
                let file_stat = FileSystem::file_nix_stat(&log_file);
                let stat_time = format!("{}|{}", file_stat.atime, file_stat.mtime);
                file_stat_map.log_files.insert(log_file.clone(), stat_time.into());

                // save the log file's current state of bytes
                match FileSystem::read(&log_file) {
                    Ok(contents) => {
                        FileSystem::write(
                            &save_state_file,
                            &contents
                        )?
                    },
                    Err(error) => {
                        // log the file if it's not authorized to access
                        if error.to_string().contains("Permission denied") {
                            push_message(
                                Type::Skipped,
                                &format!("Logging `{}` requires sudo privileges.", log_file)
                            )
                        } else {
                            push_message(
                                Type::Error,
                                &format!("Couldn't read `{}` because: {}.", log_file, error)
                            )
                        }
                    }
                };
            }
        }

        // save a JSON map of all log file's unix timestamps
        let json_config = serde_json::to_string_pretty(&file_stat_map)?;
        let save_path = format!("{}/{}", *TMP_LOG_DIR, "log_file_timestamps.json");
        FileSystem::write(&save_path, json_config.as_bytes())?;

        push_message(Type::Success, "Saved the current log states.");

        Ok(())
    }

    /// Restore the saved state of log files to clear the modification traces
    pub fn restore_state() -> Result<()> {
        // get current authenticated user
        let user = &values::CURR_USER;

        // retrieve timestamps of files
        let read_log_json = FileSystem::read(
            &format!("{}/{}", *TMP_LOG_DIR, "log_file_timestamps.json")
        )?;

        let file_stat_map: StatMap = serde_json::from_slice(&read_log_json)?;

        for log_file in values::LOG_FILES {
            let mut log_file: String = String::from(log_file);

            // parse and resolve `~/` home path
            if log_file.starts_with('~') {
                let current_user = format!(
                    "/home/{:?}/",
                    user.name()
                ).replace('"', "");

                log_file = log_file.replace("~/", &current_user);
            }

            let fmt_filename = log_file.clone();

            let read_path = format!("{}{}", *TMP_LOG_DIR, fmt_filename)
                .replace('~', "");

            if FileSystem::file_exists(&read_path) {
                // restore the initial states of all logged files
                match FileSystem::read(&read_path) {
                    Ok(contents) => {
                        match FileSystem::write(
                            &log_file,
                            &contents
                        ) {
                            Ok(()) => (),
                            Err(error) => {
                                if error.to_string().contains("Permission denied") {
                                    push_message(
                                        Type::Skipped,
                                        &format!("Writing `{}` requires sudo privileges.", log_file)
                                    )
                                } else {
                                    push_message(
                                        Type::Error,
                                        &format!("Couldn't write `{}` because: {}.", log_file, error)
                                    )
                                }
                            }
                        }
                    },
                    Err(error) => {
                        push_message(
                            Type::Error,
                            &format!("Couldn't read `{}` because: {}.", log_file, error)
                        )
                    }
                };

                // resolve timestamps of files from stat map
                let timestamps: Vec<&str> = file_stat_map.log_files.get(&log_file).unwrap()
                    .as_str()
                    .unwrap()
                    .split('|')
                    .collect();

                let atime = timestamps[0];
                let mtime = timestamps[1];

                let file_stats = FileStat {
                    atime: atime.into(),
                    mtime: mtime.into(),
                    ctime: String::new()
                };

                FileSystem::change_file_timestamp(
                    &log_file,
                    file_stats
                )
            }
        }

        push_message(Type::Success, "Restored initial machine states.");

        Ok(())
    }
}