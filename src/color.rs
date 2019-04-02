use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn from(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.r, self.g, self.b)
    }
}

impl Default for Color {
    fn default() -> Self {
        Color { r: 0, g: 0, b: 0 }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct ImpactColor {
    r: i16,
    g: i16,
    b: i16
}

impl ImpactColor {
    pub fn from(r: i16, g: i16, b: i16) -> Self {
        ImpactColor { r, g, b }
    }
}

impl fmt::Display for ImpactColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.r, self.g, self.b)
    }
}
