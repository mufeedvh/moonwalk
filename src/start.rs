use std::env;
use std::io::Result;

use colored::*;

use crate::core::{
    clear::clear_me_from_history,
    logger::Logger,
    fs::FileSystem,
    messages::{Type, push_message}
};

/// CLI interface invoking user commands
pub fn init() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = args[1].as_str();

        match command {
            "start" => {
                // save current machine log states
                Logger::init()?;
            },
            "finish" => {
                // restore machine state to how it was
                Logger::restore_state()?;
                // clear every invokation of moonwalk from shell history
                clear_me_from_history()?;
            },
            "get" => {
                if args.len() > 2 {
                    let filename = args[2].as_str();
                    let file_stats = FileSystem::file_nix_stat(filename);

                    let command = format!(
                        "touch -a -t {} -m -t {} {}",
                        file_stats.atime,
                        file_stats.mtime,
                        filename
                    );

                    let prefix = format!("{}{}{}", "[".bold(), ">".bold().cyan(), "]".bold());
                    eprintln!(
                        "\n{} To restore the access/modify timestamp of this file, use command ↓\n\n $ {}\n",
                        prefix,
                        command.magenta()
                    );

                    // clear every invokation of moonwalk from shell history
                    clear_me_from_history()?;
                } else {
                    push_message(
                        Type::Error,
                        "Please specify the filename to get it's timestamp change command."
                    )
                }
            },
            _ => ()
        }
    } else {
        // print a banner to look cool
        eprintln!(
            "{}",
            "
            ┌┬┐┌─┐┌─┐┌┐┌┬ ┬┌─┐┬  ┬┌─
            ││││ ││ │││││││├─┤│  ├┴┐
            ┴ ┴└─┘└─┘┘└┘└┴┘┴ ┴┴─┘┴ ┴ v1.0.0
            ".red()
        );

        eprintln!(
            "{}\n\n{}{}\n{}{}\n{}{}\n",
            "\nUsage".bold().cyan(),
            "Start moonwalk:".bold().magenta(),
            "\n\n\t$ moonwalk start\n".bold(),
            "Finish moonwalk and clear your traces:".bold().magenta(),
            "\n\n\t$ moonwalk finish\n".bold(),
            "Get the current timestamp of a file to restore it later:".bold().magenta(),
            "\n\n\t$ moonwalk get <FILENAME>".bold()
        )
    }

    Ok(())
}