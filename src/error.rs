#[derive(Debug, Clone)]
pub enum DmGuiError {
    DMX(String),
    Evaluation(String),
    Networking(String),
}

impl DmGuiError {
    pub fn dmx<S: Into<String>>(s: S) -> Self {
        Self::DMX(s.into())
    }
    pub fn evaluation<S: Into<String>>(s: S) -> Self {
        Self::Evaluation(s.into())
    }
    pub fn networking<S: Into<String>>(s: S) -> Self {
        Self::Networking(s.into())
    }
}

use std::fmt;
impl fmt::Display for DmGuiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DMX(s) => write!(f, "DMX Error: {s}"),
            Self::Evaluation(s) => write!(f, "DMX Error: {s}"),
            Self::Networking(s) => write!(f, "DMX Error: {s}"),
        }
    }
}

impl From<std::io::Error> for DmGuiError {
    fn from(e: std::io::Error) -> Self {
        Self::networking(format!("Networking error {e}"))
    }
}

impl From<artnet_protocol::Error> for DmGuiError {
    fn from(e: artnet_protocol::Error) -> Self {
        Self::Networking(format!("Error with using artnet: {e}"))
    }
}
