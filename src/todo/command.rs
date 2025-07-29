use crate::{
    arg::{TuduArg, ValidDateTime, ValidUrl, parse_required_project_id},
    display::{Display, Prefix},
    error::{TuduError, TuduResult},
    infrastructure::database,
    project::sql::Project,
    schema::todos::dsl as todos_dsl,
    todo::sql::{CloseTodo, NewTodo, Todo, TodoPriority, TodoStatus, UpdateTodo},
};
use clap::{ArgMatches, Command};
use diesel::{
    BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl, RunQueryDsl, insert_into,
    update,
};

pub fn new_todo_command() -> Command {
    Command::new("todo").args([
        TuduArg::Title.into_arg(false).required(true),
        TuduArg::ProjectId.into_arg(false),
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
    let project_id = parse_required_project_id(matches)?;
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
        project_id,
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

    res.to_message(Some(Prefix::New)).display();
    Ok(())
}

pub fn update_todo_command() -> Command {
    Command::new("todo").args([
        TuduArg::TodoId.into_arg(false),
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
        .get_one(TuduArg::TodoId.name())
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

    res.to_message(Some(Prefix::Update)).display();
    Ok(())
}

pub fn close_todo_command() -> Command {
    Command::new("todo").args([TuduArg::TodoId.into_arg(false).required(true)])
}

fn parse_close_todo_command_matches(matches: &ArgMatches) -> TuduResult<CloseTodo> {
    let id: &i32 = matches
        .get_one(TuduArg::TodoId.name())
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

    res.to_message(Some(Prefix::Close)).display();
    Ok(())
}

pub fn view_todo_command() -> Command {
    Command::new("todo").args([TuduArg::TodoId.into_arg(false).required(true)])
}

fn parse_view_todo_command_matches(matches: &ArgMatches) -> TuduResult<i32> {
    let id: &i32 = matches
        .get_one(TuduArg::TodoId.name())
        .ok_or_else(|| TuduError::RequiredArgumentError)?;

    Ok(id.clone())
}

pub fn handle_view_todo_command(matches: &ArgMatches) -> TuduResult<()> {
    use crate::schema::projects::dsl as projects_dsl;

    let mut connection = database::database_connection();
    let view_todo_id = parse_view_todo_command_matches(matches)?;

    let (todo, todo_children, project) = connection.transaction(
        move |conn| -> Result<(Todo, Vec<Todo>, Project), diesel::result::Error> {
            let todo = todos_dsl::todos
                .filter(todos_dsl::id.eq(view_todo_id))
                .first::<Todo>(conn)?;
            let direct_children = todos_dsl::todos
                .filter(todos_dsl::parent_id.eq(view_todo_id))
                .load::<Todo>(conn)?;
            let project = projects_dsl::projects
                .filter(projects_dsl::id.eq(todo.project_id))
                .first::<Project>(conn)?;
            Ok((todo, direct_children, project))
        },
    )?;

    project.to_message(None).display();
    todo.to_detailed_message(None).display();
    for child in todo_children {
        child.to_detailed_message(None).display();
    }

    Ok(())
}

pub fn list_todo_command() -> Command {
    Command::new("todo").args([
        TuduArg::Priority.into_arg(true),
        TuduArg::IncludeDone.into_arg(true),
    ])
}

struct ListTodoFilters {
    priority: TodoPriority,
    include_done: bool,
}

fn parse_list_todo_command_matches(matches: &ArgMatches) -> TuduResult<ListTodoFilters> {
    let priority = matches
        .get_one(TuduArg::Priority.name())
        .copied()
        .unwrap_or_default();
    let include_done = matches.get_flag(TuduArg::IncludeDone.name());
    Ok(ListTodoFilters {
        priority,
        include_done,
    })
}

pub fn handle_list_todo_command(matches: &ArgMatches) -> TuduResult<()> {
    let mut connection = database::database_connection();
    let filters = parse_list_todo_command_matches(matches)?;

    let res: Vec<Todo> = connection.transaction(move |conn| {
        if !filters.include_done {
            todos_dsl::todos
                .filter(
                    todos_dsl::status
                        .ne(TodoStatus::Done)
                        .and(todos_dsl::priority.eq(filters.priority))
                        .or(todos_dsl::priority.gt(filters.priority)),
                )
                .load::<Todo>(conn)
        } else {
            todos_dsl::todos
                .filter(
                    todos_dsl::priority
                        .eq(filters.priority)
                        .or(todos_dsl::priority.gt(filters.priority)),
                )
                .load::<Todo>(conn)
        }
    })?;

    for todo in res {
        println!();
        todo.to_message(None).display();
    }
    Ok(())
}
