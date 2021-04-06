use crate::errors::{AppResult, Error};
use bindings::windows::devices::wi_fi::*;
use regex::Regex;
use run_script::ScriptOptions;

pub async fn connected_ssid() -> AppResult<String> {
    let wifi_adapters = WiFiAdapter::find_all_adapters_async()?
        .await?
        .into_iter();
    let mut ssid = "".into();

    for adapter in wifi_adapters {
        let connection_profile = adapter
            .network_adapter()?
            .get_connected_profile_async()?
            .await?
            .wlan_connection_profile_details()?;
        ssid = connection_profile
            .get_connected_ssid()?
            .to_string();
    }

    if ssid.is_empty() {
        Err(Error::SsidMissing)
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
        Err(Error::SsidNotFound)
    } else {
        let re = Regex::new(r#"Key\sContent\s+:\s(.*)"#).unwrap();

        let password = re
            .captures(&output)
            .ok_or(Error::PasswordNotFound)?
            .get(1)
            .ok_or(Error::PasswordNotFound)?
            .as_str();

        if password.is_empty() {
            Err(Error::PasswordNotFound)
        } else {
            Ok(password.to_string())
        }
    }
}
