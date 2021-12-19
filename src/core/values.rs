use once_cell::sync::Lazy;
use users::{get_user_by_uid, get_current_uid};

pub static CURR_USER: Lazy<users::User> = Lazy::new(|| {
    get_user_by_uid(get_current_uid()).unwrap()
});

/// A list of all the common logging files in a UNIX machine
pub static LOG_FILES: [&str; 23] = [
    "~/.bash_history",
    "~/.zsh_history",
    "~/Library/Logs/DiagnosticReports",
    "~/Library/Logs",
    "/var/log/messages",
    "/var/log/auth.log",
    "/var/log/kern.log",
    "/var/log/cron.log",
    "/var/log/maillog",
    "/var/log/boot.log",
    "/var/log/mysqld.log",
    "/var/log/qmail",
    "/var/log/httpd",
    "/var/log/lighttpd",
    "/var/log/secure",
    "/var/log/utmp",
    "/var/log/lastlog",
    "/var/log/wtmp",
    "/var/log/yum.log",
    "/var/log/system.log",
    "/var/log/DiagnosticMessages",
    "Library/Logs",
    "Library/Logs/DiagnosticReports"
]; // Thanks https://github.com/sundowndev/covermyass