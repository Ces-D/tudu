use tudu::{
    arg::TuduArg,
    cli,
    display::{error_message, migration_message},
    error::TuduError,
    infrastructure::database,
    project::command::{
        handle_close_project_command, handle_list_project_command, handle_new_project_command,
        handle_update_project_command, handle_view_project_command,
    },
    todo::command::{
        handle_close_todo_command, handle_list_todo_command, handle_new_todo_command,
        handle_update_todo_command, handle_view_todo_command,
    },
};

struct CommandProcessor;

impl CommandProcessor {
    pub fn new() -> Self {
        Self {}
    }
    fn handle_migrations_command(&mut self) {
        match database::run_database_migrations() {
            Ok(migration_count) => migration_message(migration_count),
            Err(err) => error_message(err),
        }
    }

    fn process_subcommands(&mut self, cmd_matches: &clap::ArgMatches, action: &str) {
        let result = match cmd_matches.subcommand() {
            Some(("todo", todo_matches)) => match action {
                "new" => handle_new_todo_command(todo_matches),
                "update" => handle_update_todo_command(todo_matches),
                "close" => handle_close_todo_command(todo_matches),
                "view" => handle_view_todo_command(todo_matches),
                "list" => handle_list_todo_command(todo_matches),
                _ => unreachable!(),
            },
            Some(("project", project_matches)) => match action {
                "new" => handle_new_project_command(project_matches),
                "update" => handle_update_project_command(project_matches),
                "close" => handle_close_project_command(project_matches),
                "view" => handle_view_project_command(project_matches),
                "list" => handle_list_project_command(),
                _ => unreachable!(),
            },
            Some((_, _)) => Err(TuduError::CommandNotFoundError),
            None => Err(TuduError::CommandRequiredError),
        };

        if let Err(error) = result {
            error_message(error)
        }
    }
}

fn main() {
    let m = cli().get_matches();
    let prompt_arg: Option<&String> = m.get_one(TuduArg::Prompt.name());

    if prompt_arg.is_some() {
        error_message(TuduError::InProgressError);
        return;
    }

    let mut processor = CommandProcessor::new();

    match m.subcommand() {
        Some(("migrations", _)) => processor.handle_migrations_command(),
        Some(("new", cmd_matches)) => processor.process_subcommands(cmd_matches, "new"),
        Some(("update", cmd_matches)) => processor.process_subcommands(cmd_matches, "update"),
        Some(("close", cmd_matches)) => processor.process_subcommands(cmd_matches, "close"),
        Some(("view", cmd_matches)) => processor.process_subcommands(cmd_matches, "view"),
        Some(("list", cmd_matches)) => processor.process_subcommands(cmd_matches, "list"),
        Some((_, _)) => error_message(TuduError::CommandNotFoundError),
        None => error_message(TuduError::CommandRequiredError),
    }
}
