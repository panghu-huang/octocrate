use crate::{error::GithubError, ExpirableToken};
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct GithubAPIClient<T: ExpirableToken + Clone> {
    token: T,
}

pub struct GithubAPIRequest<T: Serialize + DeserializeOwned + ?Sized> {
    request_builder: RequestBuilder,
    _marker: PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned + ?Sized> GithubAPIRequest<T> {
    pub fn new(request_builder: RequestBuilder) -> Self {
        GithubAPIRequest {
            request_builder,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.request_builder = self.request_builder.header(key.into(), value.into());

        self
    }

    #[allow(dead_code)]
    pub fn query<Q>(mut self, query: &Q) -> Self
    where
        Q: Serialize + ?Sized,
    {
        self.request_builder = self.request_builder.query(query);

        self
    }

    #[allow(dead_code)]
    pub fn body<R>(mut self, json: &R) -> Self
    where
        R: Serialize + ?Sized,
    {
        self.request_builder = self.request_builder.json(json);

        self
    }

    pub async fn send(self) -> Result<T, GithubError> {
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
        GithubAPIClient { token }
    }

    pub fn get<R>(&self, url: impl Into<String>) -> GithubAPIRequest<R>
    where
        R: Serialize + DeserializeOwned + ?Sized,
    {
        let builder = Client::new().get(url.into());
        let builder = self.format_request(builder);

        GithubAPIRequest::new(builder)
    }

    pub fn post<R>(&self, url: impl Into<String>) -> GithubAPIRequest<R>
    where
        R: Serialize + DeserializeOwned + ?Sized,
    {
        let builder = Client::new().post(url.into());
        let builder = self.format_request(builder);

        GithubAPIRequest::new(builder)
    }

    pub fn put<R>(&self, url: impl Into<String>) -> GithubAPIRequest<R>
    where
        R: Serialize + DeserializeOwned + ?Sized,
    {
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
