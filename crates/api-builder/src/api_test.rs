use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{ParseBuffer, ParseStream},
    Error, Expr, Ident,
};

/// # Example
/// ```rust
///
/// api! {
///   GithubRepositoriesAPI {
///     list {
///       method GET
///       path "https://api.github.com/user/repos"
///       params {
///         page u64
///         state String
///       }
///       test {
///         params {
///            page 1
///            state "all".to_string()
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
    pub assert: Expr,
}

#[derive(Debug, Clone)]
pub struct APITestBuilder {
    pub name: Ident,
    pub struct_name: Ident,
    pub params: Option<Vec<Expr>>,
    pub assert: Option<Expr>,
}

impl APITestBuilder {
    pub fn new(struct_name: Ident, name: Ident) -> Self {
        Self {
            name,
            struct_name,
            params: None,
            assert: None,
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

    pub fn build(&mut self) -> APITest {
        if self.assert.is_none() {
            panic!("assert is required for test `{}`", self.name);
        }

        APITest {
            name: self.name.clone(),
            struct_name: self.struct_name.clone(),
            params: self.params.take().unwrap_or(vec![]),
            assert: self.assert.take().unwrap(),
        }
    }
}

impl APITest {
    pub fn generate_ast(&self) -> TokenStream {
        let name = &self.name;
        let struct_name = &self.struct_name;
        let params = &self.params;
        let assert = &self.assert;

        quote! {
          #[tokio::test]
          async fn #name() -> GithubResult<()> {
            let envs = test_utils::load_test_envs()?;
            let api_client = test_utils::create_api_client()?;

            let api = #struct_name::new(Arc::new(api_client));

            let res = api
                .#name(
                    #(#params),*
                )
                .await?;

            #assert;

            Ok(())
          }
        }
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
