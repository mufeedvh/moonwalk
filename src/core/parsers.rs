use std::borrow::Cow;
use super::fs::FileStat;

/// Parses the date time format from `stat` to `[[CC]YY]MMDDhhmm[.ss]` format
#[inline]
pub fn nix_timestamp_parser(
    timestamp_str: &str
) -> String {
    let mut fmt_time = String::with_capacity(15);
    let mut start_parse: bool = false;
    let mut seek_colon: bool = false;

    for c in timestamp_str.chars() {
        if c == ' ' { start_parse = true }
        if start_parse {
            match c {
                '-' | ' ' => (),
                ':' => {
                    if seek_colon {
                        fmt_time.push('.')
                    }
                    seek_colon = true;
                },
                '.' => break,
                _ => fmt_time.push(c)
            }
        }
    }

    fmt_time
} // hehe

/// Offloads the required fields from `stat` to parse timestamps
#[inline]
pub fn nix_stat_parser(
    stream: Cow<'_, str>
) -> FileStat {
    let mut atime = String::with_capacity(15);
    let mut mtime = String::with_capacity(15);
    let ctime = String::new();

    for line in stream.lines() {
        if line.contains("Access") && !line.contains("Uid") {
            atime = nix_timestamp_parser(line);
        } else if line.contains("Modify") {
            mtime = nix_timestamp_parser(line)
        }
    }

    FileStat {
        atime,
        mtime,
        ctime
    }
}