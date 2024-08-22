use std::collections::HashMap;

use eyre::{bail, Result};
use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default)]
/// NocoDB client.
pub struct NocoDB {
    base_url: String,
    api_key: String,
    http_client: reqwest::blocking::Client,
}

impl NocoDB {
    /// Create a new NocoDB client.
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            base_url,
            api_key,
            http_client: reqwest::blocking::Client::new(),
        }
    }

    /// Insert multiple records into a table.
    pub fn insert_bulk(
        &self,
        table_id: &str,
        data: Vec<HashMap<String, String>>,
    ) -> Result<Vec<i32>> {
        let url = format!("{}/api/v2/tables/{}/records", self.base_url, table_id);

        let res: Response = self
            .http_client
            .post(&url)
            .header("xc-token", self.api_key.clone())
            .json(&data)
            .send()?;

        if false == res.status().is_success() {
            bail!("Error while inserting data: {:?}", res.text()?);
        }

        let res: Vec<PostRecordOutput> = res.json()?;

        let ids: Vec<i32> = res.iter().map(|x| x.id).collect();

        Ok(ids)
    }
}

// -- DTOS --

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PostRecordOutput {
    #[serde(rename = "Id")]
    id: i32,
}
