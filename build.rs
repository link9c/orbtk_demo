#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    if std::path::Path::new("res/icon.ico").exists() {
        let mut res = winres::WindowsResource::new();
        res.set_icon("res/icon.ico");
        res.compile().expect("Unable to find visual studio tools");
    } else {
        panic!("No Icon.ico found. Please add one or check the path");
    }
}

#[cfg(not(windows))]
fn main() {}
