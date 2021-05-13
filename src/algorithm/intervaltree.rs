use std::borrow::Cow;

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