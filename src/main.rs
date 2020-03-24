use std::process::Command;
use structopt::StructOpt;

/// Uses MacOS airport and keychain CLI tools to obtain the Wi-Fi passwords.
#[derive(StructOpt, Debug)]
#[structopt(name = "macos wifi password")]
struct Opt {
    /// Specify an SSID.  Defaults to currently connected Wi-Fi.
    #[structopt(short, long)]
    ssid: Option<String>,
}

fn main() {
    let opt = Opt::from_args();
    let ssid = match opt.ssid {
        Some(ssid_in) => ssid_in.to_owned(),
        None => connected_ssid(),
    };
    let password = password_from_keychain(ssid);

    println!("{}", password);
}

fn connected_ssid() -> String {
    let airport_path =
        "/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport";
    let output = Command::new(airport_path)
        .arg("-I")
        .output()
        .expect("Airport not found");
    let airport_info = String::from_utf8(output.stdout).expect("Not UTF-8");
    airport_info
        .lines()
        .filter(|x| x.contains("SSID"))
        .last()
        .expect("SSID not found")
        .split("SSID:")
        .last()
        .expect("SSID not found")
        .trim()
        .to_owned()
}

fn password_from_keychain(ssid: String) -> String {
    let output = Command::new("security")
        .arg("find-generic-password")
        .args(&["-D", "AirPort network password"])
        .args(&["-ga", &ssid])
        .output()
        .expect("process failed to execute");
    let password_raw = String::from_utf8(output.stderr).expect("Not UTF-8");
    password_raw
        .lines()
        .last()
        .expect("Password not found")
        .split("password:")
        .last()
        .expect("Password not found")
        .trim()
        .replace("\"", "")
}
