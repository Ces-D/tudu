use std::{env, path::PathBuf, str::FromStr};

pub struct AIModelEnv {
    api_key: String,
    model: String,
}

/// Gets the AI model configuration from the environment variables.
///
/// This function looks for the `TUDU_AI_MODEL_KEY` and `TUDU_AI_API_KEY`
/// environment variables.
///
/// # Panics
///
/// This function will panic if either `TUDU_AI_MODEL_KEY` or
/// `TUDU_AI_API_KEY` environment variables are not set.
pub fn ai_model_env() -> AIModelEnv {
    let model =
        env::var("TUDU_AI_MODEL_KEY").expect("TUDU_AI_MODEL_KEY environment variable not set");
    let api_key =
        env::var("TUDU_AI_API_KEY").expect("TUDU_AI_API_KEY environment variable not set");
    AIModelEnv { api_key, model }
}

/// Gets the database path from the environment variables.
///
/// This function looks for the `TUDU_DATABASE_URL` environment variable.
/// If the variable is not set, it defaults to `$HOME/tudu.db`.
///
/// # Panics
///
/// This function will panic if the `TUDU_DATABASE_URL` is not valid unicode,
/// or if `TUDU_DATABASE_URL` is not set and the `HOME` environment variable
/// is not set.
pub fn database_url_env() -> PathBuf {
    match env::var("TUDU_DATABASE_URL") {
        Ok(var) => PathBuf::from_str(var.as_str())
            .expect("Failed to parse TUDU_DATABASE_URL env variable."),
        Err(err) => match err {
            env::VarError::NotPresent => {
                let documents_dir = env::var("HOME").expect("HOME environment variable not set");
                PathBuf::from(documents_dir).join("Documents/tudu.db")
            }
            env::VarError::NotUnicode(_) => panic!("TUDU_DATABASE_URL not valid unicode"),
        },
    }
}
