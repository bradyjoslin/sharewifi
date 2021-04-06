#[cfg(target_os = "windows")]
fn main() {
    windows::build!(
        windows::devices::wifi::*,
    );
}

#[cfg(not(target_os = "windows"))]
fn main() {}