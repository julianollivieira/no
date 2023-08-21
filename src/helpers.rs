use std::io;

/// Converts a system error to a result.
pub fn convert_syserr_to_result(n: i64) -> io::Result<()> {
    n.ne(&-1)
        .then(|| ())
        .ok_or_else(|| io::Error::last_os_error())
}
