use structopt::StructOpt;

/// Quickly share Wi-Fi passwords and connection details.
#[derive(StructOpt, Debug)]
#[structopt(name = "sharewifi")]
pub struct App {
    /// Specify an SSID.  Defaults to currently connected Wi-Fi.
    #[structopt(short, long)]
    pub ssid: Option<String>,

    /// Prints Wi-Fi Network config QR Code for Android and iOS 11+
    #[structopt(short, long)]
    pub qrcode: bool,

    /// Disables future confirmation prompts to access this password. Sudo required.
    #[structopt(short, long)]
    pub always_allow: bool,

    /// Verbose output
    #[structopt(short, long)]
    pub verbose: bool,
}
