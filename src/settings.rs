use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum Operators {
    Equals,
    NotEquals,
    Contains,
    NotContains,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Variable {
    pub key: String,
    pub operator: Operators,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pagination {
    pub start_page: String,
    pub max_depth: String,
    pub page_size: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Query {
    pub name: Option<String>,
    pub variables: Option<Vec<Variable>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub log: String,
    pub token: String,
    pub org: String,
    pub output: String,
    pub query: Query,
    pub pagination: Pagination,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            // Set defaults
            .set_default("log", "warn".to_string())?
            .set_default("output", "results.json".to_string())?
            .set_default("pagination.start_page", "1".to_string())?
            .set_default("pagination.max_depth", "1".to_string())?
            .set_default("pagination.page_size", "20".to_string())?
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("settings.toml").required(false))
            // Add in settings from the environment
            // Eg.. `DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::default())
            .build()?;
        s.try_deserialize()
    }
}
