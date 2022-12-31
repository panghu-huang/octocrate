use crate::infrastructure::{error::GithubError, expirable_token::ExpirableToken};
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct GithubAPIClient {
    token: Box<dyn ExpirableToken>,
}

pub struct GithubAPIRequest {
    request_builder: RequestBuilder,
}

impl GithubAPIRequest {
    pub fn new(request_builder: RequestBuilder) -> Self {
        GithubAPIRequest { request_builder }
    }

    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.request_builder = self.request_builder.header(key.into(), value.into());

        self
    }

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

impl GithubAPIClient {
    pub fn new<T>(token: T) -> Self
    where
        T: ExpirableToken + 'static,
    {
        GithubAPIClient {
            token: Box::new(token),
        }
    }

    pub fn get(&mut self, url: impl Into<String>) -> GithubAPIRequest {
        let builder = Client::new().get(url.into());
        let builder = self.format_request(builder);

        GithubAPIRequest {
            request_builder: builder,
        }
    }

    pub fn post(&mut self, url: impl Into<String>) -> GithubAPIRequest {
        let builder = Client::new().post(url.into());
        let builder = self.format_request(builder);

        GithubAPIRequest {
            request_builder: builder,
        }
    }

    fn format_request(&mut self, request_builder: RequestBuilder) -> RequestBuilder {
        let token = self.token.get_token();
        if token.is_some() {
            return request_builder
            .header("User-Agent", "Coodev")
            .header("Authorization", format!("Bearer {}", token.unwrap()));
        }
        request_builder.header("User-Agent", "Coodev")
    }
}
