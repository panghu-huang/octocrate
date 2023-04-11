use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseBuffer, ParseStream},
    Attribute, Ident, Result,
};

use crate::request::Request;

#[derive(Debug)]
pub struct RequestContext {
    pub name: Ident,
    pub attributes: Vec<Attribute>,
    pub requests: Vec<Request>,
}

impl RequestContext {
    pub fn generate_ast(&self) -> TokenStream {
        let name = &self.name;
        let attributes = &self.attributes;
        let requests = self.requests.iter().map(|request| request.generate_ast());

        let tests = self
            .requests
            .iter()
            .map(|request| request.generate_test_ast());

        let ast = quote! {
          use std::ops::Deref;
          use octocrate_infra::{ExpirableToken, GithubAPIClient};

          #(#attributes)*
          #[derive(Clone, Debug)]
          pub struct #name {
            client: std::sync::Arc<GithubAPIClient>,
          }

          impl #name {
            pub fn new(client: std::sync::Arc<GithubAPIClient>) -> Self {
              Self { client }
            }

            #(#requests)*

          }

          #[cfg(test)]
          mod tests {
            use super::#name;
            use crate::utils::test_utils;
            use octocrate_infra::GithubResult;
            use std::sync::Arc;

            #(#tests)*

          }
        };

        ast
    }
}

impl Parse for RequestContext {
    fn parse(input: ParseStream) -> Result<Self> {
        let attributes = match input.call(Attribute::parse_outer) {
            Ok(attr) => attr,
            Err(_) => Vec::new(),
        };
        let name: Ident = input.parse()?;

        let mut requests: Vec<Request> = Vec::new();
        if !input.is_empty() {
            let content: ParseBuffer;
            braced!(content in input);

            while !content.is_empty() {
                let request = Request::parse(name.clone(), &content as ParseStream)?;

                requests.push(request);
            }
        }

        Ok(RequestContext {
            name,
            requests,
            attributes,
        })
    }
}
