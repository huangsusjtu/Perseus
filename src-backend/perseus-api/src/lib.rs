use std::str::FromStr;
use std::time::Duration;

use anyhow::Context;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct PerseusClient {
    url: url::Url,
    client: reqwest::Client,
}

impl PerseusClient {
    pub fn new(s: &str) -> Self {
        // http://127.0.0.1/
        let url = url::Url::from_str(s).unwrap();
        let client = reqwest::ClientBuilder::new()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(1))
            .build()
            .unwrap();
        PerseusClient { url, client }
    }

    pub async fn list_map(&self) -> anyhow::Result<ListMapReply> {
        post_req(
            &self.client,
            self.url.join("map/list").unwrap(),
            &NonRequest,
        )
        .await
    }

    pub async fn list_scenario(&self) -> anyhow::Result<ListScenarioReply> {
        post_req(
            &self.client,
            self.url.join("scenario/list").unwrap(),
            &NonRequest,
        )
        .await
    }

    pub async fn list_world(&self) -> anyhow::Result<ListWorldReply> {
        post_req(
            &self.client,
            self.url.join("simulator/list").unwrap(),
            &NonRequest,
        )
        .await
    }

    pub async fn get_world(&self, name: String) -> anyhow::Result<WorldInfo> {
        post_req(
            &self.client,
            self.url.join("simulator/create").unwrap(),
            &CreateWorldRequest { name },
        )
        .await
    }

    pub async fn set_map(&self, name: String) -> anyhow::Result<WorldInfo> {
        post_req(
            &self.client,
            self.url.join("simulator/create").unwrap(),
            &CreateWorldRequest { name },
        )
        .await
    }
    pub async fn set_scenario(
        &self, name: String,
    ) -> anyhow::Result<WorldInfo> {
        post_req(
            &self.client,
            self.url.join("simulator/create").unwrap(),
            &CreateWorldRequest { name },
        )
        .await
    }

    pub async fn start(&self, name: String) -> anyhow::Result<WorldInfo> {
        post_req(
            &self.client,
            self.url.join("simulator/create").unwrap(),
            &CreateWorldRequest { name },
        )
        .await
    }

    pub async fn stop(&self, name: String) -> anyhow::Result<WorldInfo> {
        post_req(
            &self.client,
            self.url.join("simulator/create").unwrap(),
            &CreateWorldRequest { name },
        )
        .await
    }
}

/// 发送post请求
async fn post_req<T, E>(
    client: &reqwest::Client, url: url::Url, body: &T,
) -> anyhow::Result<E>
where
    T: Serialize,
    E: DeserializeOwned,
{
    let resp = client
        .post(url)
        .json(body)
        .send()
        .await
        .context("send request error")?;
    let status = resp.status();
    let data = resp.bytes().await.unwrap_or_default();
    if !status.is_success() {
        return Err(anyhow::anyhow!(status.as_str().to_string()));
    }
    if let Ok(r) = serde_json::from_slice::<E>(&data) {
        return Ok(r);
    }
    // 解析错误
    if let Ok(err) = serde_json::from_slice::<UniformError>(&data) {
        return Err(anyhow::anyhow!(err.message));
    }
    Err(anyhow::anyhow!("PARSE_ERROR",))
}

#[derive(Serialize, Debug)]
pub struct NonRequest;

#[derive(Deserialize, Debug)]
pub struct ListMapReply {
    pub list: Vec<MapInfo>,
}
#[derive(Deserialize, Debug)]
pub struct MapInfo {
    pub name: String,
}
#[derive(Deserialize, Debug)]
pub struct ListScenarioReply {
    pub list: Vec<ScenarioInfo>,
}
#[derive(Deserialize, Debug)]
pub struct ScenarioInfo {
    pub name: String,
}

#[derive(Serialize, Debug)]
pub struct CreateWorldRequest {
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct WorldInfo {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct ListWorldReply {
    pub list: Vec<WorldInfo>,
}

#[derive(Deserialize, Debug)]
pub struct UniformError {
    pub code: u16,
    pub reason: String,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
