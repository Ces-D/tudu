use crate::{
    arg::{TuduArg, ValidHexColor, parse_required_project_id},
    display::{Display, Prefix},
    error::{TuduError, TuduResult},
    infrastructure::database,
    project::sql::{NewProject, Project, UpdateProject},
    schema::projects::dsl as projects_dsl,
    todo::{
        group::organize_todos_hierarchically,
        sql::{Todo, TodoStatus},
    },
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

    res.to_message(Some(Prefix::New)).display();
    Ok(())
}

pub fn update_project_command() -> Command {
    Command::new("project").args([
        TuduArg::ProjectId.into_arg(false),
        TuduArg::Name.into_arg(true),
        TuduArg::Description.into_arg(true),
        TuduArg::Color.into_arg(true),
    ])
}

fn parse_update_project_command_matches(matches: &ArgMatches) -> TuduResult<UpdateProject> {
    let id = parse_required_project_id(matches)?;
    let name: Option<&String> = matches.get_one(TuduArg::Name.name());
    let description: Option<&String> = matches.get_one(TuduArg::Description.name());
    let color: Option<&ValidHexColor> = matches.get_one(TuduArg::Color.name());

    Ok(UpdateProject {
        id,
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

    res.to_message(Some(Prefix::Update)).display();
    Ok(())
}

pub fn close_project_command() -> Command {
    Command::new("project").args([TuduArg::ProjectId.into_arg(false)])
}

fn parse_close_project_command_matches(matches: &ArgMatches) -> TuduResult<i32> {
    parse_required_project_id(matches)
}

pub fn handle_close_project_command(matches: &ArgMatches) -> TuduResult<()> {
    let mut connection = database::database_connection();
    let close_project = parse_close_project_command_matches(matches)?;

    let res = connection.transaction(move |conn| {
        delete(projects_dsl::projects.filter(projects_dsl::id.eq(close_project))).execute(conn)
    })?;

    crate::display::simple_heading(
        format!("Deleted {}: Project {}", res, close_project),
        Some("#ff0000".to_string()),
    );
    Ok(())
}

pub fn view_project_command() -> Command {
    Command::new("project").args([TuduArg::ProjectId.into_arg(false)])
}

pub fn parse_view_project_command_matches(matches: &ArgMatches) -> TuduResult<i32> {
    parse_required_project_id(matches)
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
                .filter(todos_dsl::status.ne(TodoStatus::Done))
                .load::<Todo>(conn)?;
            Ok((project, todos))
        },
    )?;

    project.to_message(None).display();

    // Organize todos hierarchically and display them
    let todo_groups = organize_todos_hierarchically(todos);
    for group in todo_groups {
        group.to_message(None).display();
    }

    Ok(())
}

pub fn list_project_command() -> Command {
    Command::new("project")
}

pub fn handle_list_project_command() -> TuduResult<()> {
    let mut connection = database::database_connection();

    let res: Vec<Project> = connection.transaction(move |conn| {
        projects_dsl::projects
            .order(projects_dsl::created_at.desc())
            .load::<Project>(conn)
    })?;

    for project in res {
        project.to_detailed_message(None).display();
    }

    Ok(())
}
