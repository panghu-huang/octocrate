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

#[cfg(feature = "pagination")]
#[derive(Clone)]
pub struct GitHubPaginatedResponse<ResponseData> {
  pub data: ResponseData,
  pub headers: HeaderMap,
  pub status: StatusCode,
  pub version: Version,
  pub content_length: Option<u64>,
  pub url: Url,
  pub pages: octocrate_types::LinkedPages,
}

#[cfg(feature = "pagination")]
impl<ResponseData: serde::de::DeserializeOwned + IntoIterator>
  GitHubPaginatedResponse<ResponseData>
{
  pub fn paginate(self) -> octocrate_types::PaginatedData<ResponseData> {
    octocrate_types::PaginatedData {
      data: self.data,
      pages: self.pages,
    }
  }
}

#[cfg(feature = "pagination")]
impl<ResponseData: serde::de::DeserializeOwned + IntoIterator> From<GitHubResponse<ResponseData>>
  for GitHubPaginatedResponse<ResponseData>
{
  fn from(value: GitHubResponse<ResponseData>) -> Self {
    let GitHubResponse::<ResponseData> {
      data,
      headers,
      status,
      version,
      content_length,
      url,
    } = value;

    Self {
      data,
      headers: headers.clone(),
      status,
      version,
      content_length,
      url,
      pages: headers
        .get("link")
        .and_then(|header| header.to_str().ok())
        .map(From::from)
        .unwrap_or(Default::default()),
    }
  }
}
