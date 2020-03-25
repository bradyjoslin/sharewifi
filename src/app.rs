use structopt::StructOpt;

/// Uses MacOS airport and keychain CLI tools to obtain the Wi-Fi passwords.
#[derive(StructOpt, Debug)]
#[structopt(name = "Wi-Fi Password")]
pub struct App {
    /// Specify an SSID.  Defaults to currently connected Wi-Fi.
    #[structopt(short, long)]
    pub ssid: Option<String>,

    /// Prints Wi-Fi Network config QR Code for Android and iOS 11+
    #[structopt(short, long)]
    pub qrcode: bool,

    /// Verbose output
    #[structopt(short, long)]
    pub verbose: bool,
}
