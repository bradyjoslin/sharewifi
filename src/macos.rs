use regex::Regex;
use run_script::ScriptOptions;
use security_framework::os::macos::keychain::SecKeychain;
use std::io;

use crate::errors::{AppResult, Error};
use io::Write;

pub fn connected_ssid() -> AppResult<String> {
    let (_, output, _) = run_script::run_script!(
        r#"
            /System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport -I
        "#
    )
    .expect("Airport not found");

    let re = Regex::new(r#"\bSSID:\s?(.*)"#).unwrap();

    let ssid = re
        .captures(&output)
        .ok_or(Error::SSIDMissing)?
        .get(1)
        .ok_or(Error::SSIDMissing)?
        .as_str();

    if ssid.is_empty() {
        Err(Error::SSIDMissing)
    } else {
        Ok(ssid.to_string())
    }
}

pub fn get_password(ssid: &str) -> AppResult<String> {
    let keychain = "/Library/Keychains/System.keychain";
    let service = "AirPort";
    let res = SecKeychain::open(keychain)
        .unwrap()
        .find_generic_password(service, ssid);

    match res {
        Ok((password, _)) => {
            let password = String::from_utf8(password.to_owned()).unwrap();
            if password.is_empty() {
                Err(Error::PasswordNotFound)
            } else {
                Ok(password)
            }
        }
        Err(err) if err.code() == -25300 => Err(Error::SSIDNotFound),
        Err(err) => {
            println!("This happened: {} - {}\nSSID: {}", err.code(), err, ssid);
            Err(Error::KeychainAccess)
        }
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
        let options = ScriptOptions::new();

        let (code, _, _) = run_script::run_script!(
            r#"
                security add-generic-password -U -a "$1" \
                -D "AirPort network password" -T "/usr/bin/security" \
                -s "AirPort" /Library/Keychains/System.keychain
            "#,
            &vec![ssid.to_string()],
            options
        )
        .expect("Problem calling security tool");

        match code {
            0 => {
                println!("Keychain updated...\n");
                Ok(())
            }
            _ => Err(Error::KeyChainWriteAccess),
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
