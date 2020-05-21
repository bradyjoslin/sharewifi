use qrcode::render::unicode;
use qrcode::QrCode;
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
    let password = password_from_keychain(&ssid)?;

    if app.verbose {
        println!("SSID: {}\nPassword: {}", ssid, password);
    } else if app.qrcode {
        let image = qrcode(&ssid, &password);
        println!("{}", image);
    } else {
        println!("{}", password);
    }

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

fn password_from_keychain(ssid: &str) -> AppResult<String> {
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

fn qrcode(ssid: &str, password: &str) -> String {
    let code = QrCode::new(format!("WIFI:T:WPA;S:{};P:{};;", &ssid, &password)).unwrap();
    code.render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build()
}
