use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{ParseBuffer, ParseStream},
    Error, Expr, Ident,
};

type Object = Vec<(Ident, Expr)>;

/// # Example
/// ```rust
/// extern crate octocrate_api_builder;
/// use octocrate_api_builder::github_api;
/// 
/// pub struct Response {}
///
/// github_api! {
///   GithubRepositoriesAPI {
///     list {
///       method GET
///       path "https://api.github.com/user/repos"
///       params {
///         page u64
///         state String
///       }
///       response Response
///       test {
///         params {
///            1
///            "all".to_string()
///         }
///         assert assert_eq!(res.len(), 30)
///       }
///     }
///   }
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct APITest {
    pub name: Ident,
    pub struct_name: Ident,
    pub params: Vec<Expr>,
    pub query: Option<Object>,
    pub body: Option<Object>,
    pub ignore: bool,
    pub assert: Expr,
}

#[derive(Debug, Clone)]
pub struct APITestBuilder {
    pub name: Ident,
    pub struct_name: Ident,
    pub params: Option<Vec<Expr>>,
    pub query: Option<Object>,
    pub body: Option<Object>,
    pub ignore: bool,
    pub assert: Option<Expr>,
}

impl APITestBuilder {
    pub fn new(struct_name: Ident, name: Ident) -> Self {
        Self {
            name,
            struct_name,
            params: None,
            assert: None,
            query: None,
            ignore: false,
            body: None,
        }
    }

    pub fn params(&mut self, params: Vec<Expr>) -> &mut Self {
        self.params = Some(params);
        self
    }

    pub fn assert(&mut self, assert: Expr) -> &mut Self {
        self.assert = Some(assert);
        self
    }

    pub fn query(&mut self, query: Object) -> &mut Self {
        self.query = Some(query);
        self
    }

    pub fn body(&mut self, body: Object) -> &mut Self {
        self.body = Some(body);
        self
    }

    pub fn build(&mut self) -> APITest {
        if self.assert.is_none() {
            panic!("assert is required for test `{}`", self.name);
        }

        APITest {
            name: self.name.clone(),
            struct_name: self.struct_name.clone(),
            params: self.params.take().unwrap_or(vec![]),
            assert: self.assert.take().unwrap(),
            query: self.query.take(),
            ignore: self.ignore,
            body: self.body.take(),
        }
    }
}

impl APITest {
    pub fn generate_ast(&self) -> TokenStream {
        let name = &self.name;
        let struct_name = &self.struct_name;
        let params = &self.params;
        let query = &self.generate_query_ast();
        let body = &self.generate_body_ast();
        let ignore = &self.generate_ignore_ast();
        let assert = &self.assert;

        quote! {
          #ignore
          #[tokio::test]
          async fn #name() -> GithubResult<()> {
            let envs = test_utils::load_test_envs()?;
            let api_client = test_utils::create_api_client()?;

            let api = #struct_name::new(Arc::new(api_client));

            let res = api
                .#name(
                    #(#params),*
                )
                #query
                #body
                .send()
                .await?;

            #assert;

            Ok(())
          }
        }
    }

    fn generate_body_ast(&self) -> TokenStream {
        match &self.body {
            Some(body) => {
                let body = body
                    .iter()
                    .map(|(key, value)| {
                        let key = key.to_string();
                        quote! {
                            #key: #value
                        }
                    })
                    .collect::<Vec<TokenStream>>();

                quote! {
                    .body(&serde_json::json!({
                        #(#body),*
                    }))
                }
            }
            None => {
                quote! {}
            }
        }
    }

    fn generate_ignore_ast(&self) -> TokenStream {
        if self.ignore {
            quote! {
                #[ignore]
            }
        } else {
            quote! {}
        }
    }

    fn generate_query_ast(&self) -> TokenStream {
        match &self.query {
            Some(query) => {
                let query = query
                    .iter()
                    .map(|(key, value)| {
                        let key = key.to_string();
                        quote! {
                           (#key, #value),
                        }
                    })
                    .collect::<Vec<TokenStream>>();

                quote! {
                    .query(&[#(#query),*])
                }
            }
            None => {
                quote! {}
            }
        }
    }

    fn parse_object(input: ParseStream) -> syn::Result<Object> {
        let content: ParseBuffer;
        braced!(content in input);

        let mut object: Object = vec![];

        while !content.is_empty() {
            let key: Ident = content.parse()?;
            let value: Expr = content.parse()?;

            object.push((key, value));
        }

        Ok(object)
    }

    pub fn parse(struct_name: Ident, name: Ident, input: ParseStream) -> syn::Result<APITest> {
        let content: ParseBuffer;
        braced!(content in input);

        let mut test = APITestBuilder::new(struct_name, name);

        while !content.is_empty() {
            let key: Ident = content.parse()?;
            match key.to_string().as_str() {
                "params" => {
                    let block: ParseBuffer;
                    braced!(block in content);

                    let mut params: Vec<Expr> = vec![];

                    while !block.is_empty() {
                        let value: Expr = block.parse()?;

                        params.push(value);
                    }
                    test.params(params);
                }
                "assert" => {
                    let exp: Expr = content.parse()?;

                    test.assert(exp);
                }
                "query" => {
                    let object = Self::parse_object(&content)?;

                    test.query(object);
                }
                "body" => {
                    let object = Self::parse_object(&content)?;

                    test.body(object);
                }
                "ignore" => {
                    test.ignore = true;
                }
                _ => {
                    return Err(Error::new_spanned(
                        &key,
                        "Invalid key for `test` block. Valid keys are: `params`, `assert`",
                    ));
                }
            }
        }

        Ok(test.build())
    }
}
