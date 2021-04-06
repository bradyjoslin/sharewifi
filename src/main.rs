use qrcode::render::unicode;
use qrcode::QrCode;
use structopt::StructOpt;

mod app;
mod errors;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
use crate::windows::*;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
use crate::macos::*;

use errors::AppResult;

#[tokio::main]
async fn main() -> AppResult<()> {
    let app = app::App::from_args();
    let ssid = match &app.ssid {
        Some(ssid_in) => ssid_in.to_owned(),
        #[cfg(target_os = "windows")]
        None => connected_ssid().await?,
        #[cfg(target_os = "macos")]
        None => connected_ssid()?,
    };
    let password = get_password(&ssid)?;

    #[cfg(target_os = "macos")]
    if app.always_allow {
        always_allow(&ssid)?;
    }

    match app {
        app::App { verbose: true, .. } => println!("SSID: {}\nPassword: {}", ssid, password),
        app::App { qrcode: true, .. } => {
            let image = qrcode(&ssid, &password);
            println!("{}", image);
        }
        _ => println!("{}", password),
    }

    Ok(())
}

fn qrcode(ssid: &str, password: &str) -> String {
    let code = QrCode::new(format!("WIFI:T:WPA;S:{};P:{};;", &ssid, &password)).unwrap();
    code.render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build()
}
