use crate::{
  api_config::SharedAPIConfig,
  error::{APIErrorResponse, Error},
  no_content_request_builder::NoContentRequestBuilder,
};
use std::marker::PhantomData;

pub struct NoContentRequest<Body, Query> {
  pub(crate) builder: reqwest::RequestBuilder,
  pub(crate) api_config: SharedAPIConfig,
  pub(crate) _body: PhantomData<Body>,
  pub(crate) _query: PhantomData<Query>,
}

impl<Body, Query> NoContentRequest<Body, Query>
where
  Body: serde::Serialize,
  Query: serde::Serialize,
{
  pub fn builder(config: &SharedAPIConfig) -> NoContentRequestBuilder<Body, Query> {
    NoContentRequestBuilder::new(config)
  }

  pub fn query(mut self, query: &Query) -> Self {
    self.builder = self.builder.query(query);

    self
  }

  pub fn body(mut self, body: &Body) -> Self {
    self.builder = self.builder.json(body);

    self
  }

  pub async fn send(self) -> Result<(), Error> {
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

          let err_msg = format!("Request failed with {}", status);
          return Err(Error::Error(err_msg));
        }

        Ok(())
      }
      Err(err) => Err(Error::Error(err.to_string())),
    }
  }
}
