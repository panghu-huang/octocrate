use crate::{api_config::SharedAPIConfig, no_content_request::NoContentRequest};
use reqwest::Client;
use std::marker::PhantomData;

pub struct NoContentRequestBuilder<Body, Query> {
  builder: Option<reqwest::RequestBuilder>,
  api_config: SharedAPIConfig,
  _body: PhantomData<Body>,
  _query: PhantomData<Query>,
}

impl<Body, Query> NoContentRequestBuilder<Body, Query>
where
  Body: serde::Serialize,
  Query: serde::Serialize,
{
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      builder: None,
      api_config: config.clone(),
      _body: PhantomData,
      _query: PhantomData,
    }
  }

  pub fn get(mut self, url: impl Into<String>) -> Self {
    let url = format!("{}{}", self.api_config.base_url, url.into());
    let builder = Client::new().get(url);

    self.builder = Some(builder);

    self
  }

  pub fn post(mut self, url: impl Into<String>) -> Self {
    let url = format!("{}{}", self.api_config.base_url, url.into());
    let builder = Client::new().post(url);

    self.builder = Some(builder);

    self
  }

  pub fn put(mut self, url: impl Into<String>) -> Self {
    let url = format!("{}{}", self.api_config.base_url, url.into());
    let builder = Client::new().put(url);

    self.builder = Some(builder);

    self
  }

  pub fn patch(mut self, url: impl Into<String>) -> Self {
    let url = format!("{}{}", self.api_config.base_url, url.into());
    let builder = Client::new().patch(url);

    self.builder = Some(builder);

    self
  }

  pub fn delete(mut self, url: impl Into<String>) -> Self {
    let url = format!("{}{}", self.api_config.base_url, url.into());
    let builder = Client::new().delete(url);

    self.builder = Some(builder);

    self
  }

  pub fn build(self) -> NoContentRequest<Body, Query> {
    NoContentRequest {
      builder: self.builder.expect("RequestBuilder is not configured"),
      api_config: self.api_config.clone(),
      _body: PhantomData,
      _query: PhantomData,
    }
  }
}
