use crate::{
    arg::{TuduArg, ValidDateTime, ValidUrl},
    display::{
        Prefix, detailed_todo_message, simple_heading, simple_todo_message,
        simple_todo_message_with_prefix,
    },
    error::{TuduError, TuduResult},
    infrastructure::database,
    schema::todos::dsl as todos_dsl,
    todo::sql::{CloseTodo, NewTodo, Todo, TodoPriority, TodoStatus, UpdateTodo},
};
use clap::{ArgMatches, Command};
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, insert_into, update};

pub fn new_todo_command() -> Command {
    Command::new("todo").args([
        TuduArg::ProjectId.into_arg(false).required(true),
        TuduArg::Title.into_arg(false).required(true),
        TuduArg::ParentId.into_arg(true),
        TuduArg::Description.into_arg(true),
        TuduArg::Priority.into_arg(true),
        TuduArg::DueDate.into_arg(true),
        TuduArg::EstimatedMinutes.into_arg(true),
        TuduArg::Location.into_arg(true),
        TuduArg::Url.into_arg(true),
    ])
}

fn parse_new_todo_command_matches(matches: &ArgMatches) -> TuduResult<NewTodo> {
    let project_id: &i32 = matches
        .get_one(TuduArg::ProjectId.name())
        .ok_or_else(|| TuduError::RequiredArgumentError)?;
    let title: &String = matches
        .get_one(TuduArg::Title.name())
        .ok_or_else(|| TuduError::RequiredArgumentError)?;
    let parent_id: Option<&i32> = matches.get_one(TuduArg::ParentId.name());
    let description: Option<&String> = matches.get_one(TuduArg::Description.name());
    let priority: Option<&TodoPriority> = matches.get_one(TuduArg::Priority.name());
    let due_date: Option<&ValidDateTime> = matches.get_one(TuduArg::DueDate.name());
    let estimated_minutes: Option<&i32> = matches.get_one(TuduArg::EstimatedMinutes.name());
    let location: Option<&String> = matches.get_one(TuduArg::Location.name());
    let url: Option<&ValidUrl> = matches.get_one(TuduArg::Url.name());

    Ok(NewTodo {
        project_id: project_id.clone(),
        title: title.to_owned(),
        parent_id: parent_id.copied(),
        description: description.map(|s| s.to_owned()),
        priority: priority.copied().unwrap_or_default(),
        due_date: due_date.map(|d| d.0),
        estimated_minutes: estimated_minutes.map(|m| m.to_owned()),
        location: location.map(|s| s.to_owned()),
        url: url.map(|u| u.0.to_string()),
        status: TodoStatus::default(),
        created_at: None,
        updated_at: None,
        completed_at: None,
    })
}

pub fn handle_new_todo_command(matches: &ArgMatches) -> TuduResult<()> {
    let mut connection = database::database_connection();
    let new_todo = parse_new_todo_command_matches(matches)?;

    let res = connection.transaction(move |conn| {
        if new_todo.parent_id.is_some() {
            let parents_parent_id = todos_dsl::todos
                .filter(todos_dsl::id.eq(new_todo.parent_id.unwrap()))
                .select(todos_dsl::parent_id)
                .first::<Option<i32>>(conn)?;
            if parents_parent_id.is_some() {
                return Err(TuduError::UnSupportedError(
                    "Sub sub todos are not supported.".to_string(),
                ));
            }
        }
        insert_into(todos_dsl::todos)
            .values(new_todo)
            .get_result::<Todo>(conn)
            .map_err(|e| TuduError::from(e))
    })?;

    simple_todo_message_with_prefix(res, Prefix::New);
    Ok(())
}

pub fn update_todo_command() -> Command {
    Command::new("todo").args([
        TuduArg::Id.into_arg(false).required(true),
        TuduArg::ProjectId.into_arg(true),
        TuduArg::ParentId.into_arg(true),
        TuduArg::Title.into_arg(true),
        TuduArg::Description.into_arg(true),
        TuduArg::Status.into_arg(true),
        TuduArg::Priority.into_arg(true),
        TuduArg::DueDate.into_arg(true),
        TuduArg::EstimatedMinutes.into_arg(true),
        TuduArg::Location.into_arg(true),
        TuduArg::Url.into_arg(true),
    ])
}

fn parse_update_todo_command_matches(matches: &ArgMatches) -> TuduResult<UpdateTodo> {
    let id: &i32 = matches
        .get_one(TuduArg::Id.name())
        .ok_or_else(|| TuduError::RequiredArgumentError)?;
    let project_id: Option<&i32> = matches.get_one(TuduArg::ProjectId.name());
    let parent_id: Option<&i32> = matches.get_one(TuduArg::ParentId.name());
    let title: Option<&String> = matches.get_one(TuduArg::Title.name());
    let description: Option<&String> = matches.get_one(TuduArg::Description.name());
    let status: Option<&TodoStatus> = matches.get_one(TuduArg::Status.name());
    let priority: Option<&TodoPriority> = matches.get_one(TuduArg::Priority.name());
    let due_date: Option<&ValidDateTime> = matches.get_one(TuduArg::DueDate.name());
    let estimated_minutes: Option<&i32> = matches.get_one(TuduArg::EstimatedMinutes.name());
    let location: Option<&String> = matches.get_one(TuduArg::Location.name());
    let url: Option<&ValidUrl> = matches.get_one(TuduArg::Url.name());

    Ok(UpdateTodo {
        id: id.clone(),
        project_id: project_id.copied(),
        title: title.map(|title| title.to_owned()),
        parent_id: parent_id.copied(),
        description: description.map(|s| s.to_owned()),
        priority: priority.copied(),
        due_date: due_date.map(|d| d.0),
        estimated_minutes: estimated_minutes.copied(),
        location: location.map(|s| s.to_owned()),
        url: url.map(|u| u.0.to_string()),
        status: status.copied(),
        updated_at: chrono::Utc::now().naive_utc(),
        completed_at: match status {
            Some(s) => match s {
                TodoStatus::Done => Some(chrono::Utc::now().naive_utc()),
                TodoStatus::Cancelled => Some(chrono::Utc::now().naive_utc()),
                _ => None,
            },
            None => None,
        },
    })
}

pub fn handle_update_todo_command(matches: &ArgMatches) -> TuduResult<()> {
    let mut connection = database::database_connection();
    let update_todo = parse_update_todo_command_matches(matches)?;

    let res = connection.transaction(move |conn| {
        update(todos_dsl::todos.filter(todos_dsl::id.eq(update_todo.id)))
            .set(update_todo)
            .get_result::<Todo>(conn)
    })?;

    simple_todo_message_with_prefix(res, Prefix::Update);
    Ok(())
}

pub fn close_todo_command() -> Command {
    Command::new("todo").args([TuduArg::TaskId.into_arg(false).required(true)])
}

fn parse_close_todo_command_matches(matches: &ArgMatches) -> TuduResult<CloseTodo> {
    let id: &i32 = matches
        .get_one(TuduArg::Id.name())
        .ok_or_else(|| TuduError::RequiredArgumentError)?;

    Ok(CloseTodo {
        id: id.clone(),
        updated_at: chrono::Utc::now().naive_utc(),
        status: TodoStatus::Done,
        completed_at: chrono::Utc::now().naive_utc(),
    })
}

pub fn handle_close_todo_command(matches: &ArgMatches) -> TuduResult<()> {
    let mut connection = database::database_connection();
    let close_todo = parse_close_todo_command_matches(matches)?;

    let res = connection.transaction(move |conn| {
        update(todos_dsl::todos.filter(todos_dsl::id.eq(close_todo.id)))
            .set(close_todo)
            .get_result::<Todo>(conn)
    })?;

    simple_todo_message_with_prefix(res, Prefix::Close);
    Ok(())
}

pub fn view_todo_command() -> Command {
    Command::new("todo").args([TuduArg::TaskId.into_arg(false).required(true)])
}

fn parse_view_todo_command_matches(matches: &ArgMatches) -> TuduResult<i32> {
    let id: &i32 = matches
        .get_one(TuduArg::Id.name())
        .ok_or_else(|| TuduError::RequiredArgumentError)?;

    Ok(id.clone())
}

pub fn handle_view_todo_command(matches: &ArgMatches) -> TuduResult<()> {
    use crate::schema::projects::dsl as projects_dsl;

    let mut connection = database::database_connection();
    let view_todo_id = parse_view_todo_command_matches(matches)?;

    let (todo, todo_children, (project_name, project_color)) = connection.transaction(
        move |conn| -> Result<(Todo, Vec<Todo>, (String, Option<String>)), diesel::result::Error> {
            let todo = todos_dsl::todos
                .filter(todos_dsl::id.eq(view_todo_id))
                .first::<Todo>(conn)?;
            let direct_children = todos_dsl::todos
                .filter(todos_dsl::parent_id.eq(view_todo_id))
                .load::<Todo>(conn)?;
            let project = projects_dsl::projects
                .filter(projects_dsl::id.eq(todo.project_id))
                .select((projects_dsl::name, projects_dsl::color))
                .first::<(String, Option<String>)>(conn)?;
            Ok((todo, direct_children, project))
        },
    )?;

    simple_heading(project_name, project_color);
    detailed_todo_message(todo, 0);
    for child in todo_children {
        detailed_todo_message(child, 5);
    }

    Ok(())
}

pub fn list_todo_command() -> Command {
    Command::new("todo").args([
        TuduArg::DueDate.into_arg(true),
        TuduArg::Priority.into_arg(true),
        TuduArg::Status.into_arg(true),
        TuduArg::GreaterThan.into_arg(true),
        TuduArg::LessThan.into_arg(true),
    ])
}

struct ListTodoFilters {
    due_date: Option<chrono::NaiveDateTime>,
    priority: Option<TodoPriority>,
    status: Option<TodoStatus>,
    greater_than: bool,
    less_than: bool,
}

fn parse_list_todo_command_matches(matches: &ArgMatches) -> TuduResult<ListTodoFilters> {
    let due_date: Option<&ValidDateTime> = matches.get_one(TuduArg::DueDate.name());
    let priority: Option<&TodoPriority> = matches.get_one(TuduArg::Priority.name());
    let status: Option<&TodoStatus> = matches.get_one(TuduArg::Status.name());
    let greater_than: bool = matches.get_flag(TuduArg::GreaterThan.name());
    let less_than: bool = matches.get_flag(TuduArg::LessThan.name());

    Ok(ListTodoFilters {
        due_date: due_date.map(|d| d.0),
        priority: priority.copied(),
        status: status.copied(),
        greater_than: greater_than,
        less_than: less_than,
    })
}

pub fn handle_list_todo_command(matches: &ArgMatches) -> TuduResult<()> {
    let mut connection = database::database_connection();
    let filters = parse_list_todo_command_matches(matches)?;

    let res: Vec<Todo> = connection.transaction(move |conn| {
        let mut query = todos_dsl::todos.into_boxed();

        if let Some(d_date) = filters.due_date {
            if filters.greater_than {
                query = query.filter(todos_dsl::due_date.gt(d_date));
            } else if filters.less_than {
                query = query.filter(todos_dsl::due_date.lt(d_date));
            } else {
                query = query.filter(todos_dsl::due_date.eq(d_date));
            }
        }

        if let Some(priority) = filters.priority {
            query = query.filter(todos_dsl::priority.eq(priority));
        }

        if let Some(status) = filters.status {
            query = query.filter(todos_dsl::status.eq(status));
        }

        query.load::<Todo>(conn)
    })?;

    for todo in res {
        simple_todo_message(todo);
    }
    Ok(())
}
