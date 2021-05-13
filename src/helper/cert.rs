use std::fs;
use std::io::{self, Read};

fn get_cert_path(dir: &str, name: &str, ext: &str) -> String {
    let p: String = format!("{}/{}.{}", dir, name, ext);
    return p
}

pub fn read_single_crt(dir: &str, name: &str) -> Result<String, io::Error> {
    let mut crt: String = String::new();
    fs::File::open(get_cert_path(dir, name, "pem"))?.read_to_string(&mut crt)?;
    Ok(crt)
}

pub fn read_cert_pair(dir: &str, name: &str) -> Result<(String, String), io::Error> {
    let mut crt: String = String::new();
    let mut key: String = String::new();
    fs::File::open(get_cert_path(dir, name, "pem"))?.read_to_string(&mut crt)?;
    fs::File::open(get_cert_path(dir, name, "key"))?.read_to_string(&mut key)?;
    Ok((crt, key))
}