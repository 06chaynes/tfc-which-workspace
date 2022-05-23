mod error;
mod filter;
mod settings;
mod variable;
mod workspace;

use error::FilterError;
use http_cache_surf::{
    CACacheManager, Cache, CacheMode, CacheOptions, HttpCache,
};
use miette::{IntoDiagnostic, WrapErr};
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::fs::File;
use surf::Client;
use surf_governor::GovernorMiddleware;
use workspace::Workspace;

const BASE_URL: &str = "https://app.terraform.io/api/v2";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FilteredResultInner {
    pub workspaces: Vec<Workspace>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FilteredResultOuter {
    pub query: settings::Query,
    pub result: FilteredResultInner,
}

pub struct WorkspaceVariablesOuter {
    pub workspaces: Vec<WorkspaceVariables>,
}

pub struct WorkspaceVariables {
    pub workspace: Workspace,
    pub variables: Vec<variable::Variable>,
}

fn build_governor() -> Result<GovernorMiddleware, FilterError> {
    match GovernorMiddleware::per_second(30) {
        Ok(g) => Ok(g),
        Err(e) => Err(FilterError::General(e.into_inner())),
    }
}

#[async_std::main]
async fn main() -> miette::Result<()> {
    // Get the settings for the run
    let config = Settings::new()
        .into_diagnostic()
        .wrap_err("Uh Oh, looks like a settings issue! By default I look for a settings.toml file and override with env variables.")?;

    // Build the http client with a cache and governor enabled
    let client = Client::new().with(build_governor().into_diagnostic()?).with(
        Cache(HttpCache {
            mode: CacheMode::Default,
            manager: CACacheManager::default(),
            options: Some(CacheOptions {
                shared: false,
                cache_heuristic: 0.0,
                immutable_min_time_to_live: Default::default(),
                ignore_cargo_cult: false,
            }),
        }),
    );

    // Get the workspaces
    let workspaces = workspace::get_workspaces(&config, client.clone())
        .await
        .into_diagnostic()
        .wrap_err("Aw snap, I wasn't able to pull the list of workspaces!")?;

    // Get the variables for each workspace if query variables have been provided
    if config.query.variables.is_some() {
        let mut workspaces_variables =
            WorkspaceVariablesOuter { workspaces: vec![] };
        for workspace in workspaces {
            let variables = variable::get_variables(&workspace.id, &config, client.clone())
                .await
                .into_diagnostic()
                .wrap_err("Oops, I wasn't able to pull the list of variables for the workspace!")?;
            workspaces_variables
                .workspaces
                .push(WorkspaceVariables { workspace, variables })
        }
        filter::variable(&mut workspaces_variables, &config)
            .into_diagnostic()
            .wrap_err(
                "Darn, I ran into an issue filtering the workspace list!",
            )?;
        let mut filtered_workspaces = vec![];
        for ws in workspaces_variables.workspaces {
            filtered_workspaces.push(ws.workspace);
        }
        let res = FilteredResultOuter {
            query: config.query.clone(),
            result: FilteredResultInner { workspaces: filtered_workspaces },
        };
        println!("{:#?}", &res);
        serde_json::to_writer_pretty(
            &File::create(&config.output).into_diagnostic()?,
            &res,
        )
        .into_diagnostic()
        .wrap_err("Bleh, I ran into an issue saving the output!")?;
    }
    Ok(())
}
