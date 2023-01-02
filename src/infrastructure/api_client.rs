use crate::infrastructure::{error::GithubError, expirable_token::ExpirableToken};
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug)]
pub struct GithubAPIClient<T: ExpirableToken + Clone> {
    token: T,
}

pub struct GithubAPIRequest {
    request_builder: RequestBuilder,
}

impl GithubAPIRequest {
    pub fn new(request_builder: RequestBuilder) -> Self {
        GithubAPIRequest { request_builder }
    }

    #[allow(dead_code)]
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.request_builder = self.request_builder.header(key.into(), value.into());

        self
    }

    #[allow(dead_code)]
    pub fn json<T: Serialize + ?Sized>(mut self, json: &T) -> Self {
        self.request_builder = self.request_builder.json(json);

        self
    }

    pub async fn respond_json<T>(self) -> Result<T, GithubError>
    where
        T: DeserializeOwned,
    {
        let res = self.request_builder.send().await;
        match res {
            Ok(res) => {
                let status = res.status();
                if !status.is_success() {
                    if let Ok(error_response) = res.json::<GithubError>().await {
                        return Err(error_response);
                    }
                    let err_msg = format!("Request failed with {}", status.to_string());
                    return Err(GithubError::new(err_msg));
                }

                match res.json::<T>().await {
                    Ok(response) => {
                        return Ok(response);
                    }
                    Err(error) => {
                        println!("Error: {:#?}", error);
                        return Err(GithubError::new(error.to_string()));
                    }
                }
            }
            Err(err) => {
                return Err(GithubError::new(err.to_string()));
            }
        }
    }

    #[allow(dead_code)]
    pub async fn respond_text(self) -> Result<String, GithubError> {
        let res = self.request_builder.send().await;
        match res {
            Ok(res) => {
                let status = res.status();
                if !status.is_success() {
                    if let Ok(error_response) = res.json::<GithubError>().await {
                        return Err(error_response);
                    }
                    let err_msg = format!("Request failed with {}", status.to_string());
                    return Err(GithubError::new(err_msg));
                }

                match res.text().await {
                    Ok(response) => {
                        return Ok(response);
                    }
                    Err(error) => {
                        return Err(GithubError::new(error.to_string()));
                    }
                }
            }
            Err(err) => {
                return Err(GithubError::new(err.to_string()));
            }
        }
    }
}

impl<T: ExpirableToken + Clone> GithubAPIClient<T> {
    pub fn new(token: T) -> Self
    where
        T: ExpirableToken + Clone + 'static,
    {
        GithubAPIClient {
            token,
        }
    }

    pub fn get(&self, url: impl Into<String>) -> GithubAPIRequest {
        let builder = Client::new().get(url.into());
        let builder = self.format_request(builder);

        GithubAPIRequest::new(builder)
    }

    pub fn post(&self, url: impl Into<String>) -> GithubAPIRequest {
        let builder = Client::new().post(url.into());
        let builder = self.format_request(builder);

        GithubAPIRequest::new(builder)
    }

    pub fn put(&self, url: impl Into<String>) -> GithubAPIRequest {
        let builder = Client::new().put(url.into());
        let builder = self.format_request(builder);

        GithubAPIRequest::new(builder)
    }

    fn format_request(&self, request_builder: RequestBuilder) -> RequestBuilder {
        let request_builder = request_builder
            .header("User-Agent", "Coodev")
            .header("Accept", "application/vnd.github+json");

        let token = self.token.get_token();
        if token.is_some() {
            return request_builder.header("Authorization", format!("Bearer {}", token.unwrap()));
        }

        request_builder
    }
}
