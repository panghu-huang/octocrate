use crate::{api_config::SharedAPIConfig, request::Request};
use reqwest::Client;
use std::marker::PhantomData;

pub struct RequestBuilder<Body, Query, Response> {
  builder: Option<reqwest::RequestBuilder>,
  api_config: SharedAPIConfig,
  _body: PhantomData<Body>,
  _query: PhantomData<Query>,
  _response: PhantomData<Response>,
}

impl<Body, Query, Response> RequestBuilder<Body, Query, Response>
where
  Body: serde::Serialize,
  Query: serde::Serialize,
  Response: serde::de::DeserializeOwned,
{
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      builder: None,
      api_config: config.clone(),
      _body: PhantomData,
      _query: PhantomData,
      _response: PhantomData,
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

  pub fn build(self) -> Request<Body, Query, Response> {
    Request {
      builder: self.builder.expect("RequestBuilder is not configured"),
      api_config: self.api_config.clone(),
      _body: PhantomData,
      _query: PhantomData,
      _response: PhantomData,
    }
  }
}
