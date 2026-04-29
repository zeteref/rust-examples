#[derive(Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ColorError {
    InvalidFormat,
    InvalidLength,
    InvalidHex,
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Color { r, g, b }
    }
}

impl From<Color> for (u8, u8, u8) {
    fn from(color: Color) -> Self {
        (color.r, color.g, color.b)
    }
}

impl TryFrom<&str> for Color {
    type Error = ColorError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let hex = s.strip_prefix('#').ok_or(ColorError::InvalidFormat)?;

        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ColorError::InvalidHex)?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ColorError::InvalidHex)?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ColorError::InvalidHex)?;
                Ok(Color { r, g, b })
            }
            3 => {
                let r = u8::from_str_radix(&hex[0..1], 16).map_err(|_| ColorError::InvalidHex)?;
                let g = u8::from_str_radix(&hex[1..2], 16).map_err(|_| ColorError::InvalidHex)?;
                let b = u8::from_str_radix(&hex[2..3], 16).map_err(|_| ColorError::InvalidHex)?;
                Ok(Color {
                    r: r * 17,
                    g: g * 17,
                    b: b * 17,
                })
            }
            _ => Err(ColorError::InvalidLength),
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}
