use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseBuffer, ParseStream},
    Ident, Result,
};

use crate::request::Request;

#[derive(Debug)]
pub struct RequestContext {
    pub name: Ident,
    pub requests: Vec<Request>,
}

impl RequestContext {
    pub fn generate_ast(&self) -> TokenStream {
        let name = &self.name;
        let requests = self.requests.iter().map(|request| request.generate_ast());

        let ast = quote! {
          use std::ops::Deref;
          use infrastructure::{ExpirableToken, GithubAPIClient};
          use crate::constants::GITHUB_API_BASE_URL;

          #[derive(Clone, Debug)]
          pub struct GithubRepositoryAPI<T: ExpirableToken + Clone> {
            client: std::sync::Arc<GithubAPIClient<T>>,
          }

          impl<T: ExpirableToken + Clone> #name<T> {
            pub fn new(client: std::sync::Arc<GithubAPIClient<T>>) -> Self {
              Self { client }
            }

            #(#requests)*
          }
        };

        ast
    }
}

impl Parse for RequestContext {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;

        let mut requests: Vec<Request> = Vec::new();
        if !input.is_empty() {
            let content: ParseBuffer;
            braced!(content in input);

            while !content.is_empty() {
                let request = Request::parse(&content as ParseStream)?;

                requests.push(request);
            }
        }

        Ok(RequestContext { name, requests })
    }
}
