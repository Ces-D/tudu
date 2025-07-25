use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HexColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone)]
pub enum HexColorError {
    InvalidLength,
    InvalidCharacter,
    ParseError,
}

impl fmt::Display for HexColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HexColorError::InvalidLength => write!(f, "Invalid hex color length"),
            HexColorError::InvalidCharacter => write!(f, "Invalid hex character"),
            HexColorError::ParseError => write!(f, "Failed to parse hex color"),
        }
    }
}

impl std::error::Error for HexColorError {}

impl HexColor {
    /// Creates a new HexColor from RGB values
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Converts a hex string to HexColor
    /// Supports formats: "#RRGGBB", "RRGGBB", "#RGB", "RGB"
    pub fn from_hex(hex: &str) -> Result<Self, HexColorError> {
        let hex = hex.trim_start_matches('#');

        match hex.len() {
            3 => {
                // Short format: RGB -> RRGGBB
                let chars: Vec<char> = hex.chars().collect();
                if !chars.iter().all(|c| c.is_ascii_hexdigit()) {
                    return Err(HexColorError::InvalidCharacter);
                }

                let r = u8::from_str_radix(&format!("{}{}", chars[0], chars[0]), 16)
                    .map_err(|_| HexColorError::ParseError)?;
                let g = u8::from_str_radix(&format!("{}{}", chars[1], chars[1]), 16)
                    .map_err(|_| HexColorError::ParseError)?;
                let b = u8::from_str_radix(&format!("{}{}", chars[2], chars[2]), 16)
                    .map_err(|_| HexColorError::ParseError)?;

                Ok(Self::new(r, g, b))
            }
            6 => {
                // Long format: RRGGBB
                if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
                    return Err(HexColorError::InvalidCharacter);
                }

                let r =
                    u8::from_str_radix(&hex[0..2], 16).map_err(|_| HexColorError::ParseError)?;
                let g =
                    u8::from_str_radix(&hex[2..4], 16).map_err(|_| HexColorError::ParseError)?;
                let b =
                    u8::from_str_radix(&hex[4..6], 16).map_err(|_| HexColorError::ParseError)?;

                Ok(Self::new(r, g, b))
            }
            _ => Err(HexColorError::InvalidLength),
        }
    }

    /// Converts the HexColor back to a hex string
    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    /// Returns RGB values as a tuple
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

impl fmt::Display for HexColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_hex_long_format() {
        let color = HexColor::from_hex("#3A5857").unwrap();
        assert_eq!(color.r, 58);
        assert_eq!(color.g, 88);
        assert_eq!(color.b, 87);
    }

    #[test]
    fn test_from_hex_short_format() {
        let color = HexColor::from_hex("#f00").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_from_hex_without_hash() {
        let color = HexColor::from_hex("00ff00").unwrap();
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 255);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_to_hex() {
        let color = HexColor::new(255, 128, 64);
        assert_eq!(color.to_hex(), "#ff8040");
    }

    #[test]
    fn test_to_rgb() {
        let color = HexColor::new(255, 128, 64);
        assert_eq!(color.to_rgb(), (255, 128, 64));
    }

    #[test]
    fn test_invalid_length() {
        assert!(matches!(
            HexColor::from_hex("#ff"),
            Err(HexColorError::InvalidLength)
        ));
        assert!(matches!(
            HexColor::from_hex("#ffff"),
            Err(HexColorError::InvalidLength)
        ));
    }

    #[test]
    fn test_invalid_character() {
        assert!(matches!(
            HexColor::from_hex("#gggggg"),
            Err(HexColorError::InvalidCharacter)
        ));
    }
}
