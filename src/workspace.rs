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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pagination {
    #[serde(rename = "current-page")]
    pub current_page: u32,
    #[serde(rename = "page-size")]
    pub page_size: u32,
    #[serde(rename = "prev-page")]
    pub prev_page: Option<u32>,
    #[serde(rename = "next-page")]
    pub next_page: Option<u32>,
    #[serde(rename = "total-pages")]
    pub total_pages: u32,
    #[serde(rename = "total-count")]
    pub total_count: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Meta {
    pub pagination: Pagination,
}

#[derive(Clone, Debug, Deserialize)]
struct WorkspacesResponseOuter {
    pub data: Vec<Workspace>,
    pub meta: Option<Meta>,
}

pub async fn get_workspaces(
    config: &Settings,
    client: Client,
) -> Result<Vec<Workspace>, FilterError> {
    let mut url = Url::parse_with_params(
        &format!("{}/organizations/{}/workspaces/", BASE_URL, config.org),
        &[
            ("page[number]", config.pagination.start_page.clone()),
            ("page[size]", config.pagination.page_size.clone()),
        ],
    )?;
    if let Some(name) = config.query.name.clone() {
        url = Url::parse_with_params(url.as_str(), &[("search[name]", name)])?
    }
    let req = RequestBuilder::new(Method::Get, url.clone())
        .header("Authorization", &format!("Bearer {}", config.token))
        .build();
    let mut workspace_list: WorkspacesResponseOuter =
        match client.recv_string(req).await {
            Ok(s) => serde_json::from_str::<WorkspacesResponseOuter>(&s)?,
            Err(e) => {
                return Err(FilterError::General(e.into_inner()));
            }
        };
    // Need to check pagination
    if let Some(meta) = workspace_list.meta {
        let max_depth = config.pagination.max_depth.parse::<u32>()?;
        if max_depth > 1 || max_depth == 0 {
            let current_depth: u32 = 1;
            if let Some(next_page) = meta.pagination.next_page {
                if max_depth == 0 || current_depth < max_depth {
                    let num_pages: u32 = if max_depth
                        >= meta.pagination.total_pages
                        || max_depth == 0
                    {
                        meta.pagination.total_pages
                    } else {
                        max_depth
                    };

                    // Get the next page and merge the result
                    for n in next_page..=num_pages {
                        url = Url::parse_with_params(
                            url.clone().as_str(),
                            &[("page[number]", &n.to_string())],
                        )?;
                        let req = RequestBuilder::new(Method::Get, url.clone())
                            .header(
                                "Authorization",
                                &format!("Bearer {}", config.token),
                            )
                            .build();
                        match client.recv_string(req).await {
                            Ok(s) => {
                                let mut res =
                                    serde_json::from_str::<
                                        WorkspacesResponseOuter,
                                    >(&s)?;
                                workspace_list.data.append(&mut res.data);
                            }
                            Err(e) => {
                                return Err(FilterError::General(
                                    e.into_inner(),
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(workspace_list.data)
}
