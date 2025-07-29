use std::io::Write;

use strum::EnumProperty;

use crate::error::TuduError;

mod display;
mod hex;
mod message;
mod text;

pub use display::Display;
pub use message::Prefix;

pub fn error_message(error: TuduError) {
    let name = error.get_str("Name").expect("Missing Name Property");
    let description = error
        .get_str("Description")
        .expect("Missing Description Property");
    let cta = error.get_str("Cta").expect("Missing Cta Property");

    let line = format!(
        "{} {}: {}\n{}",
        text::Text::new("✖".to_string()).error().bold(),
        text::Text::new(name.to_string()).error().bold(),
        description.to_string(),
        text::Text::new(cta.to_string()).warning().italic()
    );

    writeln!(std::io::stderr(), "{}", line).unwrap()
}

pub fn migration_message(count: usize) {
    let success = text::Text::new("Migration successful!".to_string())
        .success()
        .padding_left(5)
        .italic();
    let message = text::Text::new(format!("Ran {} migrations", count)).padding_left(5);

    writeln!(std::io::stdout(), "{}\n{}", success, message).unwrap();
}

pub fn simple_heading(name: String, color: Option<String>) {
    let heading = text::Text::new(name)
        .color(color.unwrap_or_else(|| "#2596be".to_string()))
        .bold()
        .padding_left(2);
    writeln!(std::io::stdout(), "{}", heading).unwrap();
}
