use clap::{Command, crate_description, crate_name, crate_version};

use crate::{
    arg::TuduArg,
    project::command::{
        close_project_command, list_project_command, new_project_command, update_project_command,
        view_project_command,
    },
    todo::command::{
        close_todo_command, list_todo_command, new_todo_command, update_todo_command,
        view_todo_command,
    },
};
pub mod arg;
mod config;
pub mod display;
pub mod error;
pub mod infrastructure;
pub mod project;
mod schema;
pub mod todo;

pub fn cli() -> Command {
    Command::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(TuduArg::Prompt.into_arg(false))
        .subcommand(Command::new("migrations").about(
            "Run any pending migrations to the database store. Create db if it doesnt exist",
        ))
        .subcommand(
            Command::new("new")
                .subcommand_required(true)
                .about("Create a new instance of")
                .subcommand(new_todo_command())
                .subcommand(new_project_command()),
        )
        .subcommand(
            Command::new("update")
                .about("Update an existing instance of")
                .subcommand_required(true)
                .subcommand(update_todo_command())
                .subcommand(update_project_command()),
        )
        .subcommand(
            Command::new("close")
                .about("Close an existing instance of")
                .subcommand_required(true)
                .subcommand(close_todo_command())
                .subcommand(close_project_command()),
        )
        .subcommand(
            Command::new("view")
                .about("View the details for a single instance of, optionally filtered")
                .subcommand_required(true)
                .subcommand(view_todo_command())
                .subcommand(view_project_command()),
        )
        .subcommand(
            Command::new("list")
                .about("List an overview of multiple items, optionally filtered")
                .subcommand_required(true)
                .subcommand(list_todo_command())
                .subcommand(list_project_command()),
        )
}
