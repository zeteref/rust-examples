// Day 5: Color with Trait Conversions
//
// Implement color parsing and formatting using standard Rust conversion traits.
//
// Learning goals:
//   - Implementing `From`, `TryFrom`, and `Display` traits
//   - Custom error enums with `PartialEq` and `Debug`
//   - Hex string parsing and validation
//   - Format string precision

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

// Convert a tuple of three u8 values into a Color
impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        todo!("Implement From<(u8, u8, u8)> for Color")
    }
}

// Convert a Color into a tuple of three u8 values
impl From<Color> for (u8, u8, u8) {
    fn from(color: Color) -> Self {
        todo!("Implement From<Color> for (u8, u8, u8)")
    }
}

// Parse a color from a hex string ("#FF8000" or "#F80")
impl TryFrom<&str> for Color {
    type Error = ColorError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        todo!("Implement TryFrom<&str> for Color")
    }
}

// Format a Color as uppercase hex: "#FF8000"
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("Implement Display for Color")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_tuple_creates_correct_color() {
        let color = Color::from((255u8, 128u8, 0u8));
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn color_to_tuple_conversion() {
        let color = Color { r: 255, g: 128, b: 0 };
        let (r, g, b): (u8, u8, u8) = color.into();
        assert_eq!((r, g, b), (255, 128, 0));
    }

    #[test]
    fn try_from_full_hex_succeeds() {
        let color = Color::try_from("#FF8000").unwrap();
        assert_eq!(color, Color { r: 255, g: 128, b: 0 });
    }

    #[test]
    fn try_from_short_hex_expands() {
        let color = Color::try_from("#F80").unwrap();
        // #F80 expands: F -> FF, 8 -> 88, 0 -> 00
        assert_eq!(color, Color { r: 255, g: 136, b: 0 });
    }

    #[test]
    fn try_from_lowercase_hex_succeeds() {
        let color = Color::try_from("#ff8000").unwrap();
        assert_eq!(color, Color { r: 255, g: 128, b: 0 });
    }

    #[test]
    fn try_from_missing_hash_returns_error() {
        let result = Color::try_from("FF8000");
        assert_eq!(result, Err(ColorError::InvalidFormat));
    }

    #[test]
    fn try_from_invalid_length_returns_error() {
        let result = Color::try_from("#FF80");
        assert_eq!(result, Err(ColorError::InvalidLength));
    }

    #[test]
    fn try_from_invalid_hex_chars_returns_error() {
        let result = Color::try_from("#FF80GG");
        assert_eq!(result, Err(ColorError::InvalidHex));
    }

    #[test]
    fn try_from_empty_string_returns_error() {
        let result = Color::try_from("");
        assert_eq!(result, Err(ColorError::InvalidFormat));
    }

    #[test]
    fn display_formats_uppercase() {
        let color = Color { r: 255, g: 128, b: 0 };
        assert_eq!(format!("{}", color), "#FF8000");
    }

    #[test]
    fn display_formats_black() {
        let color = Color { r: 0, g: 0, b: 0 };
        assert_eq!(format!("{}", color), "#000000");
    }

    #[test]
    fn display_formats_white() {
        let color = Color { r: 255, g: 255, b: 255 };
        assert_eq!(format!("{}", color), "#FFFFFF");
    }
}
