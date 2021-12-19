use std::process::Command;

use super::fs::FileSystem;

/// Returns world-writable directories in the target machine
pub fn return_wr_dir() -> String {
    let child_process = Command::new("/bin/find")
        .args(["/", "-maxdepth", "3", "-type", "d", "-perm", "-777"])
        .output()
        .expect("failed to execute child process");

    let output = String::from_utf8_lossy(&child_process.stdout);
    let dir_list: Vec<&str> = output.lines().collect();

    for dir_path in dir_list {
        if dir_path.contains('/') && FileSystem::file_exists(dir_path) {
            match FileSystem::create_dir(
                &format!("{}/{}", dir_path, ".MOONWALK")
            ) {
                Ok(()) => return String::from(dir_path),
                Err(_) => continue
            }
        }
    }

    String::from("/tmp") // fallback default
}