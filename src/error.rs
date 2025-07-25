use strum::EnumProperty;

pub type TuduResult<T> = Result<T, TuduError>;

#[derive(EnumProperty)]
pub enum TuduError {
    // A temporary Error since Prompt is not implemented
    #[strum(props(
        Name = "InProgressError",
        Description = "Coming soon! 🚀 The 'prompt' feature is the first step towards an AI-powered future for `tudu` and is not yet available. Please use one of the existing commands for now.",
        Cta = "Try running `tudu --help` for a list of commands"
    ))]
    InProgressError,

    #[strum(props(
        Name = "CommandNotFoundError",
        Description = "The command you entered was not found. Please check your spelling and try again.",
        Cta = "Try running `tudu --help` for a list of commands"
    ))]
    CommandNotFoundError,

    // A temporary Error since Prompt is not implemented
    #[strum(props(
        Name = "CommandRequiredError",
        Description = "A command is required. Please enter a command.",
        Cta = "Try running `tudu --help` for a list of commands"
    ))]
    CommandRequiredError,

    #[strum(props(
        Name = "RequiredArgumentError",
        Description = "A required argument is missing. Please provide the required argument.",
        Cta = "Try running `tudu --help` for a list of commands"
    ))]
    RequiredArgumentError,

    #[strum(props(
        Name = "DatabaseError",
        Description = "An error occurred in your database. {0}",
        Cta = "Please check your database configuration and try again."
    ))]
    DatabaseError(String),

    #[strum(props(
        Name = "UnSupportedError",
        Description = "Sorry! The action you tried is currently unsupported. {0}",
        Cta = "Please try again!!"
    ))]
    UnSupportedError(String),
}

impl From<diesel::result::Error> for TuduError {
    fn from(value: diesel::result::Error) -> Self {
        match value {
            diesel::result::Error::DatabaseError(_, database_error_information) => {
                Self::DatabaseError(database_error_information.message().to_string())
            }
            diesel::result::Error::NotFound => Self::DatabaseError(
                "No rows were returned by a query expected to return at least one row.".to_string(),
            ),
            diesel::result::Error::DeserializationError(error) => {
                Self::DatabaseError(error.to_string())
            }
            diesel::result::Error::SerializationError(error) => {
                Self::DatabaseError(error.to_string())
            }
            _ => Self::DatabaseError("An untracked error occurred.".to_string()),
        }
    }
}
