use std::fmt;

pub enum Error {
    KeychainAccess,
    SSIDMissing,
    PasswordNotFound,
    SSIDNotFound,
    KeyChainWriteAccess,
}

pub type AppResult<T> = Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::KeychainAccess => write!(f, "Unable to access keychain"),
            Error::PasswordNotFound => write!(f, "No password found"),
            Error::SSIDMissing => write!(
                f,
                "No SSID found.  Please connect to Wi-Fi or provide an SSID."
            ),
            Error::SSIDNotFound => write!(f, "SSID not found in keychain"),
            Error::KeyChainWriteAccess => write!(f, "Error updating keychain. Did you sudo?"),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
