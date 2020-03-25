use std::process::Command;
use structopt::StructOpt;

mod app;
mod errors;

use errors::{AppResult, Error};

fn main() -> AppResult<()> {
    let app = app::App::from_args();
    let ssid = match app.ssid {
        Some(ssid_in) => ssid_in.to_owned(),
        None => connected_ssid()?,
    };
    let password = password_from_keychain(ssid)?;

    println!("{}", password);
    Ok(())
}

fn connected_ssid() -> AppResult<String> {
    let airport_path =
        "/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport";
    let output = Command::new(airport_path)
        .arg("-I")
        .output()
        .expect("Airport not found");
    let airport_info = String::from_utf8(output.stdout).expect("Not UTF-8");
    let result = airport_info
        .lines()
        .filter(|x| x.contains("SSID"))
        .last()
        .unwrap_or_default()
        .split("SSID:")
        .last()
        .unwrap_or_default()
        .trim()
        .to_owned();
    match result.as_str() {
        "" => Err(Error::SSIDMissing),
        _ => Ok(result),
    }
}

fn password_from_keychain(ssid: String) -> AppResult<String> {
    let output = Command::new("security")
        .arg("find-generic-password")
        .args(&["-D", "AirPort network password"])
        .args(&["-ga", &ssid])
        .output();
    match output {
        Ok(s) => {
            if s.status.code().unwrap() == 44 {
                Err(Error::SSIDNotFound)
            } else if s.status.code().unwrap() != 0 {
                Err(Error::KeychainAccess)
            } else {
                let keychain_info = String::from_utf8(s.stderr).expect("Not UTF-8");
                let password = keychain_info
                    .lines()
                    .last()
                    .unwrap_or_default()
                    .split("password:")
                    .last()
                    .unwrap_or_default()
                    .trim()
                    .replace("\"", "");
                match password.as_str() {
                    "" => Err(Error::PasswordNotFound),
                    _ => Ok(password),
                }
            }
        }
        Err(e) => panic!(e),
    }
}
