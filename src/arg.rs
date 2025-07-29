use chrono::{NaiveDateTime, ParseError as ChronoError, Utc};
use clap::{Arg, ArgAction, ArgMatches, builder::NonEmptyStringValueParser, value_parser};
use std::str::FromStr;
use strum::EnumProperty;
use url::{ParseError as UrlError, Url};

use crate::{
    error::{TuduError, TuduResult},
    todo::sql::{TodoPriority, TodoStatus},
};

// Custom wrapper types for validation
#[derive(Debug, Clone)]
pub struct ValidUrl(pub Url);

#[derive(Debug, Clone)]
pub struct ValidDateTime(pub NaiveDateTime);

impl FromStr for ValidUrl {
    type Err = UrlError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Url::parse(s).map(ValidUrl)
    }
}

/// Enforces errors to just be a default value of now
impl FromStr for ValidDateTime {
    type Err = ChronoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try multiple datetime formats
        if let Ok(date) = NaiveDateTime::parse_from_str(s, "%m/%d/%y %I:%M%p") {
            return Ok(ValidDateTime(date));
        } else if let Ok(date) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
            return Ok(ValidDateTime(date));
        } else if let Ok(date) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M") {
            return Ok(ValidDateTime(date));
        } else if let Ok(date) = NaiveDateTime::parse_from_str(s, "%m/%d/%Y %H:%M:%S") {
            return Ok(ValidDateTime(date));
        } else if let Ok(date) = NaiveDateTime::parse_from_str(s, "%m/%d/%Y %H:%M") {
            return Ok(ValidDateTime(date));
        } else if let Ok(date) = NaiveDateTime::parse_from_str(s, "%d/%m/%Y %H:%M:%S") {
            return Ok(ValidDateTime(date));
        } else if let Ok(date) = NaiveDateTime::parse_from_str(s, "%d/%m/%Y %H:%M") {
            return Ok(ValidDateTime(date));
        } else {
            let now = Utc::now().naive_utc();
            Ok(ValidDateTime(now))
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValidHexColor(pub String);

impl FromStr for ValidHexColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex_regex = regex::Regex::new(r"^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$").unwrap();

        if hex_regex.is_match(s) {
            Ok(ValidHexColor(s.to_string()))
        } else {
            Err(format!(
                "Invalid hex color format: {}. Expected format: #RRGGBB or #RGB",
                s
            ))
        }
    }
}

#[derive(EnumProperty)]
pub enum TuduArg {
    #[strum(props(
        name = "prompt",
        about = "A magical prompt to bend the application to your will. (WIP)"
    ))]
    Prompt,

    #[strum(props(name = "name", about = "The name of your project. Make it memorable!"))]
    Name,

    #[strum(props(
        name = "description",
        about = "A space for all the juicy details and notes."
    ))]
    Description,

    #[strum(props(
        name = "color",
        about = "Give your project a splash of color with a hex code (e.g., #ff0000)."
    ))]
    Color,

    #[strum(props(
        name = "project_id",
        about = "The project this task belongs to. Keeps things tidy! Can be set through `.tudu` config"
    ))]
    ProjectId,

    #[strum(props(
        name = "parent_id",
        about = "The parent task if this is a subtask. For when you need to break it down."
    ))]
    ParentId,

    #[strum(props(name = "todo_id", about = "A specific todo id"))]
    TodoId,

    #[strum(props(name = "status", about = "How is this todo going"))]
    Status,

    #[strum(props(name = "title", about = "A short, snappy title for your task."))]
    Title,

    #[strum(props(
        name = "priority",
        about = "How critical is this task? Higher numbers mean more urgency."
    ))]
    Priority,

    #[strum(props(
        name = "due_date",
        about = "The deadline for this task. Don't miss it! Default to `now`"
    ))]
    DueDate,
    #[strum(props(
        name = "estimated_minutes",
        about = "Your best guess on how long this will take, in minutes."
    ))]
    EstimatedMinutes,

    #[strum(props(
        name = "location",
        about = "Where does this task need to happen? (e.g., 'Office', 'Home')."
    ))]
    Location,

    #[strum(props(
        name = "url",
        about = "A link to a related resource, like a ticket or a document."
    ))]
    Url,

    #[strum(props(name = "greater_than", about = "A greater than comparison"))]
    GreaterThan,

    #[strum(props(name = "less_than", about = "A less than comparison"))]
    LessThan,
}

impl TuduArg {
    pub fn name(&self) -> &str {
        self.get_str("name").expect("Should have name")
    }

    pub fn into_arg(self, include_long: bool) -> Arg {
        let name = self.get_str("name").expect("Should have name");
        let about = self.get_str("about").expect("Should have about");
        let arg = match self {
            TuduArg::Prompt => Arg::new(name)
                .help(about)
                .value_parser(NonEmptyStringValueParser::new()),
            TuduArg::Name => Arg::new(name)
                .help(about)
                .value_parser(NonEmptyStringValueParser::new()),
            TuduArg::Description => Arg::new(name)
                .help(about)
                .value_parser(NonEmptyStringValueParser::new()),
            TuduArg::Color => Arg::new(name)
                .help(about)
                .value_parser(value_parser!(ValidHexColor)),
            TuduArg::ProjectId => Arg::new(name).help(about).value_parser(value_parser!(i32)),
            TuduArg::ParentId => Arg::new(name).help(about).value_parser(value_parser!(i32)),
            TuduArg::TodoId => Arg::new(name).help(about).value_parser(value_parser!(i32)),
            TuduArg::Title => Arg::new(name)
                .help(about)
                .value_parser(NonEmptyStringValueParser::new()),
            TuduArg::Priority => Arg::new(name)
                .help(about)
                .default_value("low")
                .value_parser(value_parser!(TodoPriority)),

            TuduArg::DueDate => Arg::new(name)
                .help(about)
                .value_parser(value_parser!(ValidDateTime)),
            TuduArg::EstimatedMinutes => {
                Arg::new(name).help(about).value_parser(value_parser!(i32))
            }
            TuduArg::Location => Arg::new(name)
                .help(about)
                .value_parser(NonEmptyStringValueParser::new()),
            TuduArg::Url => Arg::new(name)
                .help(about)
                .value_parser(value_parser!(ValidUrl)),
            TuduArg::Status => Arg::new(name)
                .help(about)
                .default_value("to-do")
                .value_parser(value_parser!(TodoStatus)),
            TuduArg::GreaterThan => Arg::new(name).help(about).action(ArgAction::SetTrue),
            TuduArg::LessThan => Arg::new(name).help(about).action(ArgAction::SetTrue),
        };
        if include_long { arg.long(name) } else { arg }
    }
}

/// Helper function that parses arg matches and `.tudu` config for the required project_id
pub fn parse_required_project_id(matches: &ArgMatches) -> TuduResult<i32> {
    let id: Option<&i32> = matches.get_one(TuduArg::ProjectId.name());
    let project_id = match id {
        Some(id_ref) => *id_ref,
        None => match crate::config::get_project_id_from_config() {
            Some(config_id) => config_id,
            None => return Err(TuduError::RequiredArgumentError),
        },
    };
    Ok(project_id)
}
