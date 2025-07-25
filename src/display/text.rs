use std::fmt::Display;

use colored::{ColoredString, Colorize, CustomColor};

use crate::display::hex;

pub struct Text {
    text: ColoredString,
}

impl Text {
    pub fn new(text: String) -> Self {
        Self { text: text.into() }
    }

    pub fn success(mut self) -> Self {
        self.text = self.text.green();
        self
    }

    pub fn warning(mut self) -> Self {
        self.text = self.text.yellow();
        self
    }

    pub fn color(mut self, color: String) -> Self {
        let hex_color = hex::HexColor::from_hex(color.as_str()).expect("Invalid hex code");
        let (red, green, blue) = hex_color.to_rgb();
        self.text = self.text.custom_color(CustomColor::new(red, green, blue));
        self
    }

    pub fn error(mut self) -> Self {
        self.text = self.text.red();
        self
    }

    pub fn information(mut self) -> Self {
        self.text = self.text.blue();
        self
    }

    pub fn padding_left(mut self, padding: usize) -> Self {
        self.text = format!("{:>width$}", self.text, width = padding).into();
        self
    }

    pub fn padding_right(mut self, padding: usize) -> Self {
        self.text = format!("{:<width$}", self.text, width = padding).into();
        self
    }

    // pub fn padding_x(mut self, padding: usize) -> Self {
    //     self.text = format!("{:^width$}", self.text, width = padding).into();
    //     self
    // }

    pub fn bold(mut self) -> Self {
        self.text = self.text.bold();
        self
    }

    pub fn italic(mut self) -> Self {
        self.text = self.text.italic();
        self
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}
