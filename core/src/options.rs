use std::{str::FromStr, string::ToString};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "structopt")]
use structopt::StructOpt;

#[cfg_attr(feature = "structopt", derive(StructOpt))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Options {
    #[cfg_attr(feature = "structopt", structopt(long = "letterbox"))]
    pub letterbox: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self { letterbox: false }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BitmapSmoothing {
    Default,
    Always,
    Never,
}

impl Default for BitmapSmoothing {
    fn default() -> Self {
        Self::Default
    }
}

impl FromStr for BitmapSmoothing {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(Self::Default),
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            _ => Err("Invalid bitmap smoothing"),
        }
    }
}

impl ToString for BitmapSmoothing {
    fn to_string(&self) -> String {
        match self {
            Self::Default => "default",
            Self::Always => "always",
            Self::Never => "n.ever",
        }
        .to_string()
    }
}
