use crate::schema::todos;
use chrono::NaiveDateTime;
use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow, Result},
    expression::AsExpression,
    prelude::{AsChangeset, Identifiable},
    serialize::{IsNull, ToSql},
    sql_types::Integer,
    sqlite::Sqlite,
    Insertable, Queryable, Selectable,
};
use serde::{Deserialize, Serialize};
use strum::{Display, IntoStaticStr};

/// Represents the status of a todo item.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    AsExpression,
    FromSqlRow,
    IntoStaticStr,
    Display,
    clap::clap_derive::ValueEnum,
)]
#[diesel(sql_type = Integer)]
#[repr(i32)]
pub enum TodoStatus {
    /// The task has not been started.
    ToDo = 0,
    /// The task is actively being worked on.
    InProgress = 1,
    /// The task has been completed.
    Done = 2,
    /// The task is blocked by another issue.
    Blocked = 3,
    /// The task is on hold.
    OnHold = 4,
    /// The task has been cancelled.
    Cancelled = 5,
}

impl Default for TodoStatus {
    fn default() -> Self {
        Self::ToDo
    }
}

impl FromSql<Integer, Sqlite> for TodoStatus {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> Result<Self> {
        let value = i32::from_sql(bytes)?;
        match value {
            0 => Ok(TodoStatus::ToDo),
            1 => Ok(TodoStatus::InProgress),
            2 => Ok(TodoStatus::Done),
            3 => Ok(TodoStatus::Blocked),
            4 => Ok(TodoStatus::OnHold),
            5 => Ok(TodoStatus::Cancelled),
            _ => Err("Unrecognized variant".into()),
        }
    }
}

impl ToSql<Integer, Sqlite> for TodoStatus {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Sqlite>,
    ) -> diesel::serialize::Result {
        out.set_value(*self as i32);
        Ok(IsNull::No)
    }
}

/// Represents the priority level of a todo item.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    AsExpression,
    FromSqlRow,
    Display,
    clap::clap_derive::ValueEnum,
)]
#[diesel(sql_type = Integer)]
#[repr(i32)]
pub enum TodoPriority {
    /// Low priority.
    Low = 0,
    /// Medium priority.
    Medium = 1,
    /// High priority.
    High = 2,
    /// Urgent priority, requires immediate attention.
    Urgent = 3,
}

impl Default for TodoPriority {
    fn default() -> Self {
        Self::Medium
    }
}

impl FromSql<Integer, Sqlite> for TodoPriority {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> Result<Self> {
        let value = i32::from_sql(bytes)?;
        match value {
            0 => Ok(TodoPriority::Low),
            1 => Ok(TodoPriority::Medium),
            2 => Ok(TodoPriority::High),
            3 => Ok(TodoPriority::Urgent),
            _ => Err("Unrecognized variant".into()),
        }
    }
}

impl ToSql<Integer, Sqlite> for TodoPriority {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Sqlite>,
    ) -> diesel::serialize::Result {
        out.set_value(*self as i32);
        Ok(IsNull::No)
    }
}

/// Represents a new todo item to be inserted into the database.
#[derive(Debug, Insertable, Serialize)]
#[diesel(table_name = todos)]
pub struct NewTodo {
    /// The ID of the project this todo belongs to.
    pub project_id: i32,
    /// The ID of the parent todo, if this is a sub-task.
    pub parent_id: Option<i32>,
    /// The title of the todo.
    pub title: String,
    /// An optional detailed description of the todo.
    pub description: Option<String>,
    /// The current status of the todo.
    pub status: TodoStatus,
    /// The priority level of the todo.
    pub priority: TodoPriority,
    /// The optional due date for the todo.
    pub due_date: Option<NaiveDateTime>,
    /// An optional estimation of time to complete, in minutes.
    pub estimated_minutes: Option<i32>,
    /// An optional location associated with the todo.
    pub location: Option<String>,
    /// An optional URL for more information.
    pub url: Option<String>,
    /// The timestamp when the todo was created.
    pub created_at: Option<NaiveDateTime>,
    /// The timestamp when the todo was last updated.
    pub updated_at: Option<NaiveDateTime>,
    /// The timestamp when the todo was completed.
    pub completed_at: Option<NaiveDateTime>,
}

/// Represents the changes to be applied to an existing todo item.
#[derive(Debug, AsChangeset, Serialize)]
#[diesel(table_name = todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateTodo {
    /// The unique identifier of the todo to update.
    pub id: i32,
    /// The new project ID, if moving the todo.
    pub project_id: Option<i32>,
    /// The new parent ID, if changing its hierarchy.
    pub parent_id: Option<i32>,
    /// The new title for the todo.
    pub title: Option<String>,
    /// The new description for the todo.
    pub description: Option<String>,
    /// The new status for the todo.
    pub status: Option<TodoStatus>,
    /// The new priority for the todo.
    pub priority: Option<TodoPriority>,
    /// The new due date for the todo.
    pub due_date: Option<NaiveDateTime>,
    /// The new estimated time to complete, in minutes.
    pub estimated_minutes: Option<i32>,
    /// The new location for the todo.
    pub location: Option<String>,
    /// The new URL for the todo.
    pub url: Option<String>,
    /// The timestamp when the todo was last updated.
    pub updated_at: NaiveDateTime,
    /// The timestamp when the todo was completed.
    pub completed_at: Option<NaiveDateTime>,
}

/// A struct used to mark a todo as closed/completed.
#[derive(Debug, AsChangeset, Serialize)]
#[diesel(table_name = todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CloseTodo {
    /// The unique identifier of the todo to close.
    pub id: i32,
    /// The timestamp when the todo was last updated.
    pub updated_at: NaiveDateTime,
    /// The timestamp when the todo was completed.
    pub completed_at: NaiveDateTime,
    /// The final status to set for the closed todo (e.g., `Done`).
    pub status: TodoStatus,
}

/// Represents a todo item retrieved from the database.
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Todo {
    /// The unique identifier for the todo.
    pub id: i32,
    /// The ID of the project this todo belongs to.
    pub project_id: i32,
    /// The ID of the parent todo, if this is a sub-task.
    pub parent_id: Option<i32>,
    /// The title of the todo.
    pub title: String,
    /// An optional detailed description of the todo.
    pub description: Option<String>,
    /// The current status of the todo.
    pub status: TodoStatus,
    /// The priority level of the todo.
    pub priority: TodoPriority,
    /// The optional due date for the todo.
    pub due_date: Option<NaiveDateTime>,
    /// An optional estimation of time to complete, in minutes.
    pub estimated_minutes: Option<i32>,
    /// An optional location associated with the todo.
    pub location: Option<String>,
    /// An optional URL for more information.
    pub url: Option<String>,
    /// The timestamp when the todo was created.
    pub created_at: Option<NaiveDateTime>,
    /// The timestamp when the todo was last updated.
    pub updated_at: Option<NaiveDateTime>,
    /// The timestamp when the todo was completed.
    pub completed_at: Option<NaiveDateTime>,
}