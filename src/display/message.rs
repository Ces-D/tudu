use crate::display::text::Text;

#[derive(Debug, Clone, Copy)]
pub enum Prefix {
    New,
    Update,
    Close,
}

impl Prefix {
    fn to_text(&self) -> Text {
        match self {
            Prefix::New => Text::new("New".to_string()).success().bold(),
            Prefix::Update => Text::new("Updated".to_string()).information().bold(),
            Prefix::Close => Text::new("Closed".to_string()).warning().bold(),
        }
    }
}

pub struct Message {
    prefix: Option<Text>,
    pub lines: Vec<Text>,
    padding_left: Option<usize>,
}

impl Message {
    pub fn new() -> Self {
        Self {
            prefix: None,
            lines: Vec::new(),
            padding_left: None,
        }
    }

    pub fn with_padding_left(mut self, padding_left: usize) -> Self {
        self.padding_left = Some(padding_left);
        self
    }

    pub fn with_prefix(mut self, prefix: Prefix) -> Self {
        self.prefix = Some(prefix.to_text());
        self
    }

    pub fn add_line(mut self, line: Text) -> Self {
        let line = match self.padding_left {
            Some(padding) => line.padding_left(padding),
            None => line,
        };
        self.lines.push(line);
        self
    }

    pub fn display(&self) {
        if let Some(p) = &self.prefix {
            println!("{}", p);
        }

        for line in &self.lines {
            println!("{}", line);
        }
    }
}
