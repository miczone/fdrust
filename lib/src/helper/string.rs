use std::borrow::Cow;
use std::iter::once;


pub fn remove_whitespace<'a>(input: &'a str) -> Cow<'a, str> {
    if input.contains(' ') {
        return input
        .chars()
        .filter(|&x| x != ' ')
        .collect::<std::string::String>()
        .into();
    }
    return input.into();
}

#[cfg(windows)]
pub fn encode_wide(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    OsStr::new(s).encode_wide().collect()
}

#[cfg(not(windows))]
pub fn encode_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

#[cfg(windows)]
pub fn encode_wide_c(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    OsStr::new(s).encode_wide().chain(once(0)).collect()
}

#[cfg(not(windows))]
pub fn encode_wide_c(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(once(0)).collect()
}