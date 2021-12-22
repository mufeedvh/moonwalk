use std::io::Result;

use super::{
    values,
    fs::FileSystem,
    logger::TMP_LOG_DIR
};

/// Clears every invokation of `moonwalk` from shell history
pub fn clear_me_from_history() -> Result<()> {
    const HISTORY_FILES: [&str; 2] = ["~/.bash_history", "~/.zsh_history"];

    // get current authenticated user
    let user = &values::CURR_USER;

    for file in HISTORY_FILES {
        let mut file_path: String = String::from(file);

        // parse and resolve `~/` home path
        if file_path.starts_with('~') {
            let current_user = format!(
                "/home/{:?}/",
                user.name()
            ).replace('"', "");

            file_path = file_path.replace("~/", &current_user);
        }

        let mut write_buffer = String::new();

        if FileSystem::file_exists(&file_path) {
            let history_file_bytes = FileSystem::read(&file_path)?;
            
            let file_contents = String::from_utf8_lossy(
                &history_file_bytes
            );
    
            for line in file_contents.lines() {
                let condition = line.contains("moonwalk") || line.contains("MOONWALK");
    
                if !condition {
                    write_buffer.push_str(line);
                    write_buffer.push('\n')
                }
            }
    
            FileSystem::write(
                &file_path,
                write_buffer.as_bytes()
            )?;
        }
    }

    // finally remove the logging directory
    FileSystem::remove_dir(&TMP_LOG_DIR)?;

    Ok(())
}