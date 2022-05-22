use miette::Diagnostic;
use thiserror::Error;

/// A generic “error” for searches
#[derive(Error, Diagnostic, Debug)]
pub enum FilterError {
    /// A general error used as a catch all for other errors via anyhow
    #[error(transparent)]
    #[diagnostic(code(which_workspace::general))]
    General(#[from] anyhow::Error),
    /// URL parsing related errors
    #[error(transparent)]
    #[diagnostic(
        code(which_workspace::url),
        help("Oops, something went wrong building the URL!")
    )]
    Url(#[from] url::ParseError),
    /// JSON Serialization\Deserialization related errors
    #[error(transparent)]
    #[diagnostic(
        code(which_workspace::json),
        help("Aw snap, ran into an issue parsing the json response!")
    )]
    Json(#[from] serde_json::Error),
    /// Integer parsing related errors
    #[error(transparent)]
    #[diagnostic(
        code(which_workspace::int),
        help("Aw snap, ran into an issue parsing an integer!")
    )]
    Int(#[from] std::num::ParseIntError),
}
