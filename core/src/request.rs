use crate::{
  api_config::SharedAPIConfig,
  error::{APIErrorResponse, Error},
  request_builder::RequestBuilder,
};
use std::marker::PhantomData;

pub struct Request<Body, Query, Response> {
  pub(crate) builder: reqwest::RequestBuilder,
  pub(crate) api_config: SharedAPIConfig,
  pub(crate) _body: PhantomData<Body>,
  pub(crate) _query: PhantomData<Query>,
  pub(crate) _response: PhantomData<Response>,
}

impl<Body, Query, Response> Request<Body, Query, Response>
where
  Body: serde::Serialize,
  Query: serde::Serialize,
  Response: serde::de::DeserializeOwned,
{
  pub fn builder(config: &SharedAPIConfig) -> RequestBuilder<Body, Query, Response> {
    RequestBuilder::new(config)
  }

  pub fn query(mut self, query: &Query) -> Self {
    self.builder = self.builder.query(query);

    self
  }

  pub fn body(mut self, body: &Body) -> Self {
    self.builder = self.builder.json(body);

    self
  }

  pub async fn send(self) -> Result<Response, Error> {
    let mut builder = self
      .builder
      .header("User-Agent", "octocrate")
      .header("Accept", "application/vnd.github+json");

    if let Some(token) = &self.api_config.token {
      if let Some(token) = token.get_token() {
        builder = builder.header("Authorization", format!("Bearer {}", token));
      }
    }

    let res = builder.send().await;
    match res {
      Ok(res) => {
        let status = res.status();
        if !status.is_success() {
          if let Ok(error_response) = res.json::<APIErrorResponse>().await {
            return Err(Error::RequestFailed(error_response));
          }

          let err_msg = format!("Request failed with {}", status.to_string());
          return Err(Error::Error(err_msg));
        }

        let res = res.text().await.map_err(|err| {
          Error::Error(format!(
            "Failed to read response with status {}: {}",
            status, err
          ))
        })?;

        match serde_json::from_str(&res) {
          Ok(response) => {
            return Ok(response);
          }
          Err(error) => {
            return Err(Error::Error(format!(
              r#"Failed to parse response with status {}: {}

              Response: {}"#,
              status, error, res
            )));
          }
        }
      }
      Err(err) => {
        return Err(Error::Error(err.to_string()));
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::api_config::APIConfig;

  #[tokio::test]
  async fn test_request_builder() {
    let config = APIConfig::default().shared();

    #[derive(serde::Serialize)]
    struct Query {
      page: u32,
    }

    #[derive(serde::Serialize)]
    struct Body {
      name: String,
    }

    #[derive(serde::Deserialize)]
    struct Response {
      full_name: String,
    }

    let request = Request::<Body, Query, Response>::builder(&config)
      .get("/repos/panghu-huang/octocrate")
      .build();

    let response = request.query(&Query { page: 1 }).send().await.unwrap();

    assert_eq!(response.full_name, "panghu-huang/octocrate");
  }
}
