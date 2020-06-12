use winapi;
use winapi::um::shobjidl_core::FileOpenDialog;
use winapi::um::winnt::LPWSTR;

fn to_wstring(value: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;

    std::ffi::OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}
fn main() {
    let psz = 0 as *mut LPWSTR;
}
