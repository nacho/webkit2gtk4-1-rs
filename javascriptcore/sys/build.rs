// Generated by gir (https://github.com/gtk-rs/gir @ 952ff416b599)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 89a11aa6a362)
// from webkit2gtk-gir-files
// DO NOT EDIT

#[cfg(not(feature = "dox"))]
use std::process;

#[cfg(feature = "dox")]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(feature = "dox"))]
fn main() {
    if let Err(s) = system_deps::Config::new().probe() {
        println!("cargo:warning={}", s);
        process::exit(1);
    }
}
