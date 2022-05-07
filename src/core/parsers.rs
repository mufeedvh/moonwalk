use std::borrow::Cow;
use super::fs::FileStat;

/// Parses the date time format from `stat` to `[[CC]YY]MMDDhhmm[.ss]` format
#[inline]
pub fn nix_timestamp_parser(
    timestamp_str: &str
) -> String {
    timestamp_str[0..19]
	.to_string()
	.replace("-", "")
	.replace(":", "")
	.replace(" ", "")
	.replace("'", "")
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
    let mut index = 0;
    for line in stream.lines() {
	if index == 0 {
            ctime = nix_timestamp_parser(line);
	} else if index == 1 {
	    atime = nix_timestamp_parser(line);	    
	} else if index == 2 {
	    mtime = nix_timestamp_parser(line);
	}
	index = index + 1;
    }
    FileStat {
        atime,
        mtime,
        ctime
    }
}
