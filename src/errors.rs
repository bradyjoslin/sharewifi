use std::fmt;

#[cfg(target_os = "macos")]
pub enum Error {
    KeychainAccess,
    SsidMissing,
    PasswordNotFound,
    SsidNotFound,
    KeyChainWriteAccess,
}

#[cfg(target_os = "windows")]
pub enum Error {
    SsidMissing,
    PasswordNotFound,
    SsidNotFound,
}

pub type AppResult<T> = Result<T, Error>;

#[cfg(target_os = "macos")]
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::KeychainAccess => write!(f, "Unable to access keychain"),
            Error::PasswordNotFound => write!(f, "No password found"),
            Error::SsidMissing => write!(
                f,
                "No SSID found.  Please connect to Wi-Fi or provide an SSID."
            ),
            Error::SsidNotFound => write!(f, "SSID not found"),
            Error::KeyChainWriteAccess => write!(f, "Error updating keychain. Did you sudo?"),
        }
    }
}

#[cfg(target_os = "windows")]
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::PasswordNotFound => write!(f, "No password found"),
            Error::SsidMissing => write!(
                f,
                "No SSID found.  Please connect to Wi-Fi or provide an SSID."
            ),
            Error::SsidNotFound => write!(f, "SSID not found"),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
