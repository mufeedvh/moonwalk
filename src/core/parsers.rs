use std::borrow::Cow;
use super::fs::FileStat;

/// Parses the date time format from `stat` to `[[CC]YY]MMDDhhmm[.ss]` format
#[inline]
pub fn nix_timestamp_parser(
    timestamp_str: &str
) -> String {
    let mut timestamp = timestamp_str
	.to_string()
	.replace("-", "")
	.replace(":", "")
	.replace(" ", "")
	.replace("'", "");
    timestamp = timestamp[0..14].to_string();

    //YYYYMMDDhhmm.ss
    format!("{}{}{}", timestamp[0..12].to_string(), ".".to_string(), timestamp[12..14].to_string())
}

/// Offloads the required fields from `stat` to parse timestamps
#[inline]
pub fn nix_stat_parser(
    stream: Cow<'_, str>
) -> FileStat {
    let mut atime = String::with_capacity(15);
    let mut mtime = String::with_capacity(15);
    let mut ctime = String::new();

    // Each line contains one timestamp
    let mut index: usize = 0;
    for line in stream.lines() {
	match index {
	    0 => ctime = nix_timestamp_parser(line),
	    1 => atime = nix_timestamp_parser(line),
	    2 => mtime = nix_timestamp_parser(line),
	    _ => {}
	}
	index = index + 1;
    }
    FileStat {
        atime,
        mtime,
        ctime
    }
}
