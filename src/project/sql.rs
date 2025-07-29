use crate::schema::projects;
use chrono::NaiveDateTime;
use diesel::{
    Insertable, Queryable, Selectable,
    prelude::{AsChangeset, Identifiable},
};
use serde::{Deserialize, Serialize};

/// Represents a new project to be inserted into the database.
#[derive(Debug, Insertable, Serialize)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewProject {
    /// The name of the project.
    pub name: String,
    /// An optional description for the project.
    pub description: Option<String>,
    /// An optional color associated with the project (e.g., hex code).
    pub color: Option<String>,
}

/// Represents the changes to be applied to an existing project.
#[derive(Debug, AsChangeset, Serialize)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateProject {
    /// The unique identifier of the project to update.
    pub id: i32,
    /// The new name for the project, if it's being changed.
    pub name: Option<String>,
    /// The new description for the project, if it's being changed.
    pub description: Option<String>,
    /// The new color for the project, if it's being changed.
    pub color: Option<String>,
    /// The timestamp when the project was last updated.
    pub updated_at: NaiveDateTime,
}

/// Represents a project retrieved from the database.
#[derive(Debug, Queryable, Selectable, Identifiable, Deserialize)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Project {
    /// The unique identifier for the project.
    pub id: i32,
    /// The name of the project.
    pub name: String,
    /// An optional description of the project.
    pub description: Option<String>,
    /// An optional color associated with the project.
    pub color: Option<String>,
    /// The timestamp when the project was created.
    pub created_at: Option<NaiveDateTime>,
    /// The timestamp when the project was last updated.
    pub updated_at: Option<NaiveDateTime>,
}
