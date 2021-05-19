pub fn add_prefix_to_vec<'a>(prefix: &'a str, arr: &'a [&str]) -> Vec<&'a str> {
    return arr.iter().map(|item| {
        return crate::helper::string::string_to_str(format!("{}{}", &prefix, &item));
    }).collect::<Vec<_>>();
}