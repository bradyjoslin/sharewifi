use std::{io, process::Command};

use crate::errors::{AppResult, Error};
use io::Write;

pub fn connected_ssid() -> AppResult<String> {
    let airport_path =
        "/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport";
    let output = Command::new(airport_path)
        .arg("-I")
        .output()
        .expect("Airport not found");
    let airport_info = String::from_utf8(output.stdout).expect("Not UTF-8");
    let ssid = airport_info
        .lines()
        .filter(|x| x.contains("SSID"))
        .last()
        .unwrap_or_default()
        .split("SSID:")
        .last()
        .unwrap_or_default()
        .trim()
        .to_owned();
    match ssid.as_str() {
        "" => Err(Error::SSIDMissing),
        _ => Ok(ssid),
    }
}

pub fn get_password(ssid: &str) -> AppResult<String> {
    let output = Command::new("security")
        .args(&["find-generic-password", "-w"])
        .args(&["-D", "AirPort network password"])
        .args(&["-ga", ssid])
        .output();
    match output {
        Ok(o) => match o.status.code().unwrap() {
            0 => {
                let password = String::from_utf8(o.stdout)
                    .expect("Not UTF-8")
                    .trim()
                    .to_owned();
                match password.as_str() {
                    "" => Err(Error::PasswordNotFound),
                    _ => Ok(password),
                }
            }
            44 => Err(Error::SSIDNotFound),
            _ => Err(Error::KeychainAccess),
        },
        Err(e) => panic!(e),
    }
}

pub fn always_allow(ssid: &str) -> AppResult<()> {
    println!(
        "Warning: Only use always-allow for Wi-Fi passwords you don't consider secret. The password for {} in your keychain will be accessible by this app and others without credentials.", ssid);
    print!("Confirm (y/n): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.as_str().trim();

    if input == "y" {
        let output = Command::new("security")
            .args(&["add-generic-password", "-U"])
            .args(&["-a", ssid])
            .args(&["-D", "AirPort network password"])
            .args(&["-T", "/usr/bin/security"])
            .args(&["-s", "AirPort"])
            .arg("/Library/Keychains/System.keychain")
            .output();
        match output {
            Ok(o) => match o.status.code().unwrap() {
                0 => {
                    println!("Keychain updated...\n");
                    Ok(())
                }
                _ => Err(Error::KeyChainWriteAccess),
            },
            Err(e) => panic!(e),
        }
    } else if input == "n" {
        println!("Skipped keychain update...\n");
        Ok(())
    } else {
        always_allow(ssid)?;
        Ok(())
    }?;

    Ok(())
}
