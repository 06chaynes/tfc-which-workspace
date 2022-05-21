use crate::{error::FilterError, settings::Settings, BASE_URL};
use serde::{Deserialize, Serialize};
use surf::{http::Method, Client, RequestBuilder};
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Attributes {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Workspace {
    pub id: String,
    pub attributes: Attributes,
}

#[derive(Clone, Debug, Deserialize)]
struct WorkspacesResponseOuter {
    pub data: Vec<Workspace>,
}

pub async fn get_workspaces(
    config: &Settings,
    client: Client,
) -> Result<Vec<Workspace>, FilterError> {
    let mut url = Url::parse(&format!(
        "{}/organizations/{}/workspaces/",
        BASE_URL, config.org
    ))?;
    if let Some(name) = config.query.name.clone() {
        url = Url::parse_with_params(url.as_str(), &[("search[name]", name)])?
    }
    let req = RequestBuilder::new(Method::Get, url)
        .header("Authorization", &format!("Bearer {}", config.token))
        .build();
    match client.recv_string(req).await {
        Ok(s) => Ok(serde_json::from_str::<WorkspacesResponseOuter>(&s)?.data),
        Err(e) => Err(FilterError::General(e.into_inner())),
    }
}
