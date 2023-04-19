mod api_test;
mod request;
mod request_context;

use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::request_context::RequestContext;

#[proc_macro]
/// # Example
/// ```rust
/// extern crate octocrate_api_builder;
/// use octocrate_api_builder::github_api;
///
/// github_api! {
///   GithubRepositoriesAPI {
///     list {
///       method GET
///       path "/user/repos"
///       response Vec<GithubRepository>
///     }
///   }
/// }
/// ```
///
pub fn github_api(input: TokenStream) -> TokenStream {
    let ctx = parse_macro_input!(input as RequestContext);
    // println!("input: {:#?}", ctx);

    ctx.generate_ast().into()
}
