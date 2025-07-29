use chrono::NaiveDateTime;

use crate::{
    display::{
        message::{Message, Prefix},
        text::Text,
    },
    project::sql::Project,
    todo::{
        group::TodoGroup,
        sql::{Todo, TodoPriority, TodoStatus},
    },
};

const DATETIME_FORMAT: &str = "%a %b %-d, %Y %-I:%M%P";

fn format_datetime(prefix: &str, naive_date_time: NaiveDateTime) -> String {
    format!("{}: {}", prefix, naive_date_time.format(DATETIME_FORMAT))
}

pub trait Display {
    fn to_message(&self, prefix: Option<Prefix>) -> Message;
    fn to_detailed_message(&self, prefix: Option<Prefix>) -> Message;
}

fn priority_text(priority: TodoPriority) -> Text {
    let text = Text::new(format!("[P{}]", priority as i32));
    match priority {
        TodoPriority::Low => text.color("#198754".to_string()),
        TodoPriority::Medium => text.color("#0DCAF0".to_string()),
        TodoPriority::High => text.color("#FFC107".to_string()),
        TodoPriority::Urgent => text.color("#DC3545".to_string()),
    }
}

fn status_text(status: TodoStatus) -> Text {
    let text = Text::new(format!("[{}] ", status)).padding_right(15);
    match status {
        TodoStatus::ToDo => text.color("#CED4DA".to_string()),
        TodoStatus::InProgress => text.color("#0D6EFD".to_string()),
        TodoStatus::Done => text.color("#198754".to_string()),
        TodoStatus::Blocked => text.color("#DC3545".to_string()),
        TodoStatus::OnHold => text.color("#FFC107".to_string()),
        TodoStatus::Cancelled => text.color("#6C757D".to_string()),
    }
}

impl Display for Todo {
    fn to_message(&self, prefix: Option<Prefix>) -> Message {
        let id = Text::new(format!("#{}", self.id)).padding_right(5);
        let priority = priority_text(self.priority);
        let status = status_text(self.status).padding_right(20);
        let title = Text::new(self.title.clone()).padding_right(5);

        let mut line = format!("{}{}{}{}", id, priority, status, title);

        if let Some(d_date) = self.due_date {
            let d = Text::new(format_datetime("Due", d_date)).padding_left(20);
            line += d.to_string().as_str();
        }

        let mut message = Message::new().add_line(Text::new(line));

        if let Some(p) = prefix {
            message = message.with_prefix(p);
        }

        message
    }

    fn to_detailed_message(&self, prefix: Option<Prefix>) -> Message {
        let mut simple_message = self.to_message(prefix);
        let additional_lines = create_additional_lines(self);

        for line in additional_lines.into_iter() {
            simple_message = simple_message.add_line(line);
        }
        simple_message
    }
}

fn create_additional_lines(todo: &Todo) -> Vec<Text> {
    let mut lines = Vec::new();

    if let Some(desc) = &todo.description {
        lines.push(Text::new(desc.clone()));
    }

    let mut detail_parts = Vec::new();
    if let Some(location) = &todo.location {
        detail_parts.push(format!("📍 {}", location));
    }
    if let Some(url) = &todo.url {
        detail_parts.push(format!("🔗 {}", url));
    }
    if let Some(minutes) = todo.estimated_minutes {
        detail_parts.push(format!("⏱️  {}min", minutes));
    }
    if !detail_parts.is_empty() {
        lines.push(Text::new(detail_parts.join(" • ")));
    }

    let mut timestamp_parts = Vec::new();
    if let Some(created) = todo.created_at {
        timestamp_parts.push(format_datetime("Created", created));
    }
    if let Some(updated) = todo.updated_at {
        timestamp_parts.push(format_datetime("Updated", updated));
    }
    if let Some(completed) = todo.completed_at {
        timestamp_parts.push(format_datetime("Completed", completed));
    }
    if !timestamp_parts.is_empty() {
        lines.push(Text::new(timestamp_parts.join(" • ")));
    }

    lines
}

impl Display for Project {
    fn to_message(&self, prefix: Option<Prefix>) -> Message {
        let id = Text::new(format!("#{}", self.id)).padding_right(5);
        let heading = Text::new(self.name.clone())
            .color(self.color.clone().unwrap_or_else(|| "#2596be".to_string()))
            .bold()
            .padding_left(2);

        let mut message = Message::new().add_line(Text::new(format!("{}{}", id, heading)));

        if let Some(p) = prefix {
            message = message.with_prefix(p);
        }

        message
    }

    fn to_detailed_message(&self, prefix: Option<Prefix>) -> Message {
        let mut message = self.to_message(prefix);

        if let Some(desc) = &self.description {
            message = message.add_line(Text::new(desc.clone()));
        };

        message
    }
}

impl Display for TodoGroup {
    fn to_message(&self, prefix: Option<Prefix>) -> Message {
        let mut message = self.main_todo.to_message(prefix);
        println!();
        let subtodo_len = self.subtodos.len();
        for (index, subtodo) in self.subtodos.iter().enumerate() {
            if index != subtodo_len - 1 {
                println!();
            }
            let sub_message = subtodo.to_message(None).with_padding_left(4);
            for line in sub_message.lines.into_iter() {
                message = message.add_line(line);
            }
        }
        message
    }

    fn to_detailed_message(&self, prefix: Option<Prefix>) -> Message {
        let mut message = self.main_todo.to_detailed_message(prefix);
        println!();
        let subtodo_len = self.subtodos.len();
        for (index, subtodo) in self.subtodos.iter().enumerate() {
            if index != subtodo_len - 1 {
                println!();
            }
            let sub_message = subtodo.to_detailed_message(None).with_padding_left(4);
            for line in sub_message.lines.into_iter() {
                message = message.add_line(line);
            }
        }
        message
    }
}
