use regex::Regex;
use run_script::ScriptOptions;

use crate::errors::{AppResult, Error};

pub fn connected_ssid() -> AppResult<String> {
    let (_, output, _) = run_script::run_script!(
        r#"
            netsh wlan show interface
        "#
    )
    .expect("Unable to get Wi-Fi info");

    let re = Regex::new(r#"SSID\s+:\s(.*)"#).unwrap();

    let ssid = re
        .captures(&output)
        .ok_or_else(|| Error::SSIDMissing)?
        .get(1)
        .ok_or_else(|| Error::SSIDMissing)?
        .as_str();

    if ssid.is_empty() {
        Err(Error::SSIDMissing)
    } else {
        Ok(ssid.to_string())
    }
}

pub fn get_password(ssid: &str) -> AppResult<String> {
    let options = ScriptOptions::new();

    let (_, output, error) = run_script::run_script!(
        r#"
            netsh wlan show profiles name=%1 key=clear
        "#,
        &vec![ssid.to_string()],
        options
    )
    .expect("unable to get Wi-Fi info");

    if output.contains("is not found on the system") || !error.is_empty() {
        Err(Error::SSIDNotFound)
    } else {
        let re = Regex::new(r#"Key\sContent\s+:\s(.*)"#).unwrap();

        let password = re
            .captures(&output)
            .ok_or_else(|| Error::PasswordNotFound)?
            .get(1)
            .ok_or_else(|| Error::PasswordNotFound)?
            .as_str();

        if password.is_empty() {
            Err(Error::PasswordNotFound)
        } else {
            Ok(password.to_string())
        }
    }
}
