use std::io::Write;

use strum::EnumProperty;

use crate::{
    error::TuduError,
    project::sql::Project,
    todo::{group::TodoGroup, sql::Todo},
};
mod hex;
mod text;

pub fn error_message(error: TuduError) {
    let name = error.get_str("name").expect("Missing Name Property");
    let description = error
        .get_str("description")
        .expect("Missing Description Property");
    let cta = error.get_str("cta").expect("Missing Cta Property");

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

pub enum Prefix {
    New,
    Update,
    Close,
}

pub fn simple_todo_message(todo: Todo) {
    let id = text::Text::new(format!("#{}", todo.id)).padding_right(5);
    let priority = text::Text::new(format!("[P{}]", todo.priority as i32));
    let status = text::Text::new(format!("[{}] ", todo.status)).padding_right(5);
    let title = text::Text::new(todo.title).padding_right(5);

    let mut line = format!("{}{}{}{}", id, priority, status, title);

    if let Some(d_date) = todo.due_date {
        let d = text::Text::new(d_date.format("(due: %a %b %-d %-I:%M%p)").to_string());
        line += d.to_string().as_str();
    }

    writeln!(std::io::stdout(), "{}", line).unwrap();
}

pub fn simple_todo_message_with_prefix(todo: Todo, prefix: Prefix) {
    let p = match prefix {
        Prefix::New => text::Text::new("New".to_string()).success().bold(),
        Prefix::Update => text::Text::new("Updated".to_string()).information().bold(),
        Prefix::Close => text::Text::new("Closed".to_string()).warning().bold(),
    };

    writeln!(std::io::stdout(), "{}", p).unwrap();
    simple_todo_message(todo);
}

pub fn detailed_todo_message(todo: Todo, padding_left: usize) {
    let id = text::Text::new(format!("#{}", todo.id))
        .padding_right(5)
        .padding_left(padding_left);
    let priority = text::Text::new(format!("[P{}]", todo.priority as i32));
    let status = text::Text::new(format!("[{}] ", todo.status)).padding_right(5);
    let title = text::Text::new(todo.title.clone()).padding_right(5);

    let mut line = format!("{}{}{}{}", id, priority, status, title);

    if let Some(d_date) = todo.due_date {
        let d = text::Text::new(d_date.format("(due: %a %b %-d %-I:%M%p)").to_string());
        line += d.to_string().as_str();
    }
    let additional_lines = create_additional_lines(&todo, padding_left);

    // Print the main line
    writeln!(std::io::stdout(), "{}", line).unwrap();

    // Print each additional line
    for additional_line in additional_lines {
        writeln!(std::io::stdout(), "{}", additional_line).unwrap();
    }

    // Add an additional line at the end
    writeln!(std::io::stdout(), "").unwrap();
}

fn create_additional_lines(todo: &Todo, padding_left: usize) -> Vec<text::Text> {
    let mut lines = Vec::new();

    // First section: Description (gets its own line if it exists)
    if let Some(desc) = &todo.description {
        lines.push(text::Text::new(desc.clone()));
    }

    // Second section: Location, URL, and estimated_minutes (gets its own line if any exist)
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
        lines.push(text::Text::new(detail_parts.join(" • ")).padding_left(padding_left));
    }

    // Third section: Timestamps (gets its own line if any exist)
    let mut timestamp_parts = Vec::new();

    if let Some(created) = todo.created_at {
        timestamp_parts.push(format!("Created: {}", created.format("%Y-%m-%d %H:%M")));
    }

    if let Some(updated) = todo.updated_at {
        timestamp_parts.push(format!("Updated: {}", updated.format("%Y-%m-%d %H:%M")));
    }

    if let Some(completed) = todo.completed_at {
        timestamp_parts.push(format!("Completed: {}", completed.format("%Y-%m-%d %H:%M")));
    }

    if !timestamp_parts.is_empty() {
        lines.push(text::Text::new(timestamp_parts.join(" • ")).padding_left(padding_left));
    }

    lines
}

pub fn simple_project_message(project: Project) {
    let id = text::Text::new(format!("#{}", project.id)).padding_right(5);
    let heading = text::Text::new(project.name)
        .color(project.color.unwrap_or_else(|| "#2596be".to_string()))
        .bold()
        .padding_left(2);

    writeln!(std::io::stdout(), "{}{}", id, heading).unwrap();
    if let Some(desc) = project.description {
        writeln!(std::io::stdout(), "{}", desc).unwrap();
    }
}

pub fn simple_project_message_with_prefix(project: Project, prefix: Prefix) {
    let p = match prefix {
        Prefix::New => text::Text::new("New".to_string()).success().bold(),
        Prefix::Update => text::Text::new("Updated".to_string()).information().bold(),
        Prefix::Close => text::Text::new("Closed".to_string()).warning().bold(),
    };
    writeln!(std::io::stdout(), "{}", p).unwrap();
    simple_project_message(project);
}

pub fn simple_heading(name: String, color: Option<String>) {
    let heading = text::Text::new(name)
        .color(color.unwrap_or_else(|| "#2596be".to_string()))
        .bold()
        .padding_left(2);
    writeln!(std::io::stdout(), "{}", heading).unwrap();
}

pub fn display_todo_groups(todo_groups: Vec<TodoGroup>) {
    if todo_groups.is_empty() {
        return;
    }

    println!(); // Add spacing before todos

    let total_groups = todo_groups.len();
    for (index, group) in todo_groups.into_iter().enumerate() {
        let has_subtodos = !group.subtodos.is_empty();
        let is_last = index == total_groups - 1;

        // Display main todo
        detailed_todo_message(group.main_todo, 0);

        // Display subtodos with indentation
        for subtodo in group.subtodos {
            detailed_todo_message(subtodo, 4);
        }

        // Add spacing between todo groups, but not after the last one
        if !is_last && has_subtodos {
            println!();
        }
    }
}
