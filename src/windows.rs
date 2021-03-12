use std::process::Command;

use crate::errors::{AppResult, Error};

pub fn connected_ssid() -> AppResult<String> {
    let output = Command::new("netsh")
        .arg("wlan")
        .arg("show")
        .arg("interface")
        .output()
        .expect("Unable to get Wi-Fi info");

    let wifi_info = String::from_utf8(output.stdout).expect("Not UTF-8");

    let ssid = wifi_info
        .lines()
        .filter(|x| x.contains("SSID"))
        .into_iter()
        .nth(0)
        .unwrap_or_default()
        .split(":")
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
    let output = Command::new("netsh")
        .arg("wlan")
        .arg("show")
        .arg("profiles")
        .arg(format!("name={}", ssid))
        .arg("key=clear")
        .output()
        .expect("unable to get Wi-Fi info");

    let wifi_info = String::from_utf8(output.stdout).expect("Not UTF-8");

    if wifi_info.contains("is not found on the system") {
        Err(Error::SSIDNotFound)
    } else {
        let password = wifi_info
            .lines()
            .filter(|x| x.contains("Key Content"))
            .last()
            .unwrap_or_default()
            .split(":")
            .last()
            .unwrap_or_default()
            .trim()
            .to_owned();

        match password.as_str() {
            "" => Err(Error::PasswordNotFound),
            _ => Ok(password),
        }
    }
}
