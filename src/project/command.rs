use crate::{
    arg::{TuduArg, ValidHexColor},
    display::{
        Prefix, display_todo_groups, simple_heading, simple_project_message,
        simple_project_message_with_prefix,
    },
    error::{TuduError, TuduResult},
    infrastructure::database,
    project::sql::{CloseProject, NewProject, Project, UpdateProject},
    schema::projects::dsl as projects_dsl,
    todo::{group::organize_todos_hierarchically, sql::Todo},
};
use clap::{ArgMatches, Command};
use diesel::{
    Connection, ExpressionMethods, QueryDsl, RunQueryDsl,
    dsl::{delete, insert_into},
    update,
};

pub fn new_project_command() -> Command {
    Command::new("project").args([
        TuduArg::Name.into_arg(false).required(true),
        TuduArg::Description.into_arg(true),
        TuduArg::Color.into_arg(true),
    ])
}

fn parse_new_project_command_matches(matches: &ArgMatches) -> TuduResult<NewProject> {
    let name: &String = matches
        .get_one(TuduArg::Name.name())
        .ok_or_else(|| TuduError::RequiredArgumentError)?;
    let description: Option<&String> = matches.get_one(TuduArg::Description.name());
    let color: Option<&ValidHexColor> = matches.get_one(TuduArg::Color.name());

    Ok(NewProject {
        name: name.clone(),
        description: description.map(|desc| desc.clone()),
        color: color.map(|c| c.0.clone()),
    })
}

pub fn handle_new_project_command(matches: &ArgMatches) -> TuduResult<()> {
    let mut connection = database::database_connection();
    let new_project = parse_new_project_command_matches(matches)?;

    let res = connection.transaction(move |conn| {
        insert_into(projects_dsl::projects)
            .values(new_project)
            .get_result::<Project>(conn)
    })?;

    simple_project_message_with_prefix(res, Prefix::New);
    Ok(())
}

pub fn update_project_command() -> Command {
    Command::new("project").args([
        TuduArg::Id.into_arg(false).required(true),
        TuduArg::Name.into_arg(true),
        TuduArg::Description.into_arg(true),
        TuduArg::Color.into_arg(true),
    ])
}

fn parse_update_project_command_matches(matches: &ArgMatches) -> TuduResult<UpdateProject> {
    let id: &i32 = matches
        .get_one(TuduArg::Id.name())
        .ok_or_else(|| TuduError::RequiredArgumentError)?;
    let name: Option<&String> = matches.get_one(TuduArg::Name.name());
    let description: Option<&String> = matches.get_one(TuduArg::Description.name());
    let color: Option<&ValidHexColor> = matches.get_one(TuduArg::Color.name());

    Ok(UpdateProject {
        id: id.clone(),
        name: name.map(|name| name.clone()),
        description: description.map(|desc| desc.clone()),
        color: color.map(|c| c.0.clone()),
        updated_at: chrono::Utc::now().naive_utc(),
    })
}

pub fn handle_update_project_command(matches: &ArgMatches) -> TuduResult<()> {
    let mut connection = database::database_connection();
    let update_project = parse_update_project_command_matches(matches)?;

    let res = connection.transaction(move |conn| {
        update(projects_dsl::projects.filter(projects_dsl::id.eq(update_project.id)))
            .set(update_project)
            .get_result::<Project>(conn)
    })?;

    simple_project_message_with_prefix(res, Prefix::Update);
    Ok(())
}

pub fn close_project_command() -> Command {
    Command::new("project").args([TuduArg::ProjectId.into_arg(false).required(true)])
}

fn parse_close_project_command_matches(matches: &ArgMatches) -> TuduResult<CloseProject> {
    let id: &i32 = matches
        .get_one(TuduArg::Id.name())
        .ok_or_else(|| TuduError::RequiredArgumentError)?;

    Ok(CloseProject { id: id.clone() })
}

pub fn handle_close_project_command(matches: &ArgMatches) -> TuduResult<()> {
    let mut connection = database::database_connection();
    let close_project = parse_close_project_command_matches(matches)?;

    let res = connection.transaction(move |conn| {
        delete(projects_dsl::projects.filter(projects_dsl::id.eq(close_project.id))).execute(conn)
    })?;

    simple_heading(
        format!("Deleted {}: Project {}", res, close_project.id),
        Some("#ff0000".to_string()),
    );
    Ok(())
}

pub fn view_project_command() -> Command {
    Command::new("project").args([TuduArg::ProjectId.into_arg(false).required(true)])
}

pub fn parse_view_project_command_matches(matches: &ArgMatches) -> TuduResult<i32> {
    let id: &i32 = matches
        .get_one(TuduArg::ProjectId.name())
        .ok_or_else(|| TuduError::RequiredArgumentError)?;

    Ok(id.clone())
}

pub fn handle_view_project_command(matches: &ArgMatches) -> TuduResult<()> {
    use crate::schema::todos::dsl as todos_dsl;
    let mut connection = database::database_connection();
    let view_project_id = parse_view_project_command_matches(matches)?;

    let (project, todos) = connection.transaction(
        move |conn| -> Result<(Project, Vec<Todo>), diesel::result::Error> {
            let project = projects_dsl::projects
                .filter(projects_dsl::id.eq(view_project_id))
                .first::<Project>(conn)?;
            let todos = todos_dsl::todos
                .filter(todos_dsl::project_id.eq(view_project_id))
                .load::<Todo>(conn)?;
            Ok((project, todos))
        },
    )?;

    simple_project_message(project);

    // Organize todos hierarchically and display them
    let todo_groups = organize_todos_hierarchically(todos);
    display_todo_groups(todo_groups);

    Ok(())
}

pub fn list_project_command() -> Command {
    Command::new("project")
}

pub fn handle_list_project_command() -> TuduResult<()> {
    let mut connection = database::database_connection();

    let res: Vec<Project> =
        connection.transaction(move |conn| projects_dsl::projects.load::<Project>(conn))?;

    for project in res {
        simple_project_message(project);
    }

    Ok(())
}
