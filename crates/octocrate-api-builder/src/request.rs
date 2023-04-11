use crate::api_test::APITest;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{ParseBuffer, ParseStream},
    Attribute, Error, Expr, Ident, Lit, Type,
};

#[derive(Debug, Clone)]
pub enum RequestMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

#[derive(Debug, Clone)]
pub struct Request {
    name: Ident,
    attributes: Vec<Attribute>,
    method: RequestMethod,
    path: String,
    params: Vec<(Ident, Ident)>,
    response: Type,
    test: Option<APITest>,
}

#[derive(Debug)]
pub struct RequestBuilder {
    name: Ident,
    attributes: Vec<Attribute>,
    method: Option<RequestMethod>,
    path: Option<String>,
    params: Option<Vec<(Ident, Ident)>>,
    response: Option<Type>,
    test: Option<APITest>,
}

impl RequestBuilder {
    pub fn new(name: Ident, attributes: Vec<Attribute>) -> Self {
        Self {
            name,
            attributes,
            method: None,
            path: None,
            params: None,
            response: None,
            test: None,
        }
    }

    pub fn method(&mut self, method: RequestMethod) -> &mut Self {
        self.method = Some(method);
        self
    }

    pub fn path(&mut self, path: String) -> &mut Self {
        self.path = Some(path);
        self
    }

    pub fn params(&mut self, params: Vec<(Ident, Ident)>) -> &mut Self {
        self.params = Some(params);
        self
    }

    pub fn response(&mut self, response: Type) -> &mut Self {
        self.response = Some(response);
        self
    }

    pub fn test(&mut self, test: APITest) -> &mut Self {
        self.test = Some(test);
        self
    }

    pub fn build(&self) -> Request {
        if self.path.is_none() {
            panic!("Missing `path` for request: {}", self.name);
        }

        if self.response.is_none() {
            panic!("Missing `response` for request: {}", self.name);
        }
        Request {
            name: self.name.clone(),
            attributes: self.attributes.clone(),
            method: self.method.clone().unwrap_or(RequestMethod::GET),
            path: self.path.clone().unwrap(),
            params: self.params.clone().unwrap_or(vec![]),
            response: self.response.clone().unwrap(),
            test: self.test.clone(),
        }
    }
}

impl Request {
    pub fn generate_ast(&self) -> TokenStream {
        let name = &self.name;
        let response = &self.response;
        let attributes = &self.attributes;

        let params = self.generate_params_ast();
        let url = self.generate_url_ast();
        let method = self.generate_method_ast();

        let ast = quote! {
          #(#attributes)*
          pub fn #name(&self, #params) -> octocrate_infra::GithubAPIRequest<#response> {
            let url = #url;

            self.client
              .deref()
              .#method::<#response>(url)
          }
        };

        ast
    }

    pub fn generate_test_ast(&self) -> TokenStream {
        if let Some(test) = &self.test {
            test.generate_ast()
        } else {
            quote! {}
        }
    }

    fn generate_method_ast(&self) -> TokenStream {
        let method = match &self.method {
            RequestMethod::GET => quote! { get },
            RequestMethod::POST => quote! { post },
            RequestMethod::PUT => quote! { put },
            RequestMethod::DELETE => quote! { delete },
            RequestMethod::PATCH => quote! { patch },
        };

        method
    }

    fn generate_params_ast(&self) -> TokenStream {
        let params = self.params.iter().map(|(key, value)| {
            quote! {
              #key: impl Into<#value>
            }
        });

        let ast = quote! {
          #(#params),*
        };

        ast
    }

    fn generate_url_ast(&self) -> TokenStream {
        let url = &self.path;
        let args = &self
            .params
            .iter()
            .map(|(key, _)| {
                quote! {
                  #key.into()
                }
            })
            .collect::<Vec<TokenStream>>();

        let ast = quote! {
          format!(#url, #(#args),*)
        };

        ast
    }

    fn parse_params(input: ParseStream) -> syn::Result<Vec<(Ident, Ident)>> {
        let mut params = vec![];
        let content: ParseBuffer;
        braced!(content in input);

        while !content.is_empty() {
            let key: Ident = content.parse()?;
            let value: Ident = content.parse()?;

            params.push((key, value));
        }

        Ok(params)
    }

    pub fn parse(struct_name: Ident, input: ParseStream) -> syn::Result<Self> {
        let attributes = match input.call(Attribute::parse_outer) {
            Ok(attr) => attr,
            Err(_) => vec![],
        };

        let name: Ident = input.parse()?;
        let mut builder = RequestBuilder::new(name.clone(), attributes);

        let content: ParseBuffer;
        braced!(content in input);

        while !content.is_empty() {
            let key: Ident = content.parse()?;
            match key.to_string().as_str() {
                "method" => {
                    let method: Ident = content.parse()?;
                    let method = RequestMethod::from(method.to_string());

                    builder.method(method);
                }
                "path" => {
                    let v: Expr = content
                        .parse()
                        .map_err(|_| Error::new_spanned(&key, "`path` must be a string literal"))?;

                    let path = match v {
                        Expr::Lit(lit) => match lit.lit {
                            Lit::Str(s) => s.value(),
                            _ => panic!("`path` must be a string literal"),
                        },
                        _ => panic!("`path` must be a string literal"),
                    };

                    builder.path(path);
                }
                "params" => {
                    let params = Self::parse_params(&content)?;

                    builder.params(params);
                }
                "response" => {
                    let res: Type = content.parse()?;

                    builder.response(res);
                }
                "test" => {
                    let test = APITest::parse(struct_name.clone(), name.clone(), &content)?;

                    builder.test(test);
                }
                key => {
                    return Err(Error::new_spanned(key, format!("Unknown key: {}", key)));
                }
            };
        }

        Ok(builder.build())
    }
}

impl From<String> for RequestMethod {
    fn from(method: String) -> Self {
        match method.as_str() {
            "GET" | "get" => RequestMethod::GET,
            "POST" | "post" => RequestMethod::POST,
            "PUT" | "put" => RequestMethod::PUT,
            "DELETE" | "delete" => RequestMethod::DELETE,
            "PATCH" | "patch" => RequestMethod::PATCH,
            _ => {
                panic!("Unknown method: {}", method);
            }
        }
    }
}
