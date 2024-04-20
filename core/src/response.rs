use reqwest::{header::HeaderMap, StatusCode, Url, Version};

#[derive(Clone)]
pub struct GitHubResponse<ResponseData> {
  pub data: ResponseData,
  pub headers: HeaderMap,
  pub status: StatusCode,
  pub version: Version,
  pub content_length: Option<u64>,
  pub url: Url,
}
