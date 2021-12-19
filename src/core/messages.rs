use colored::*;

/// Logging message types
pub enum Type {
    _Warning,
    Skipped,
    Error,
    Info,
    Success,
}

/// Outputs logging messages
pub fn push_message(log_type: Type, message: &str) {
    let prefix = match log_type {
        Type::_Warning => format!("{}{}{}", "[".bold(), "WARN".bold().yellow(), "]".bold()),
        Type::Skipped => format!("{}{}{}", "[".bold(), "SKIPPED".bold().yellow(), "]".bold()),
        Type::Error => format!("{}{}{}", "[".bold(), "ERROR".bold().red(), "]".bold()),
        Type::Info => format!("{}{}{}", "[".bold(), "INFO".bold().cyan(), "]".bold()),
        Type::Success => format!("{}{}{}", "[".bold(), "SUCCESS".bold().green(), "]".bold())
    };

    eprintln!("{}", format!("{} {}", prefix, message))
}