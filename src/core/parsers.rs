use std::borrow::Cow;
use super::fs::FileStat;

/// Offloads the required fields from `stat` to parse timestamps
#[inline]
pub fn nix_stat_parser(
    stream: Cow<'_, str>
) -> FileStat {
    let mut atime = String::with_capacity(35);
    let mut mtime = String::with_capacity(35);
    let mut ctime = String::with_capacity(35);

    let mut index: usize = 0;
    for line in stream.lines() {
	match index {
            0 => ctime = line.to_string(),
            1 => atime = line.to_string(),
            2 => mtime = line.to_string(),
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
