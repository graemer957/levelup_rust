use std::{ffi::CString, path::Path};

// Version 1
// fn info<T: Display>(a: &T) {
//     println!("{a}");
// }

// Version 2
fn info<T: AsInfo>(a: &T) {
    println!("{}", a.as_info());
}

trait AsInfo {
    fn as_info(&self) -> String;
}

impl AsInfo for String {
    fn as_info(&self) -> String {
        self.clone()
    }
}

impl AsInfo for &str {
    fn as_info(&self) -> String {
        (*self).to_string()
    }
}

impl AsInfo for CString {
    fn as_info(&self) -> String {
        self.to_string_lossy().to_string()
    }
}

impl AsInfo for char {
    fn as_info(&self) -> String {
        self.to_string()
    }
}

impl AsInfo for &Path {
    fn as_info(&self) -> String {
        self.to_string_lossy().to_string()
    }
}

fn main() {
    let a = "?";
    let b = "?".to_string();
    info(&a);
    info(&b);

    // Advanced 1
    let c = CString::new("?").unwrap();
    info(&c);

    // Advanced 2
    let d = Path::new("/tmp/linkedin-learning");
    info(&d);
}

#[test]
fn str() {
    let input = "Rust";
    info(&input);
}

#[test]
fn string() {
    let input = String::from("Rust");
    info(&input);
}

#[test]
fn chars() {
    let input = 'r';
    info(&input);
}

#[test]
fn cstring() {
    use std::ffi::CString;
    let input = CString::new("Rust").unwrap();
    info(&input);
}

#[test]
fn path() {
    use std::path::Path;
    let input = Path::new("/tmp/rust");
    info(&input);
}
