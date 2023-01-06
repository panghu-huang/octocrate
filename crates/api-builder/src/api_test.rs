use syn::{parse_macro_input, Ident, Token, Expr};

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
pub struct APITest {
  pub name: Ident,
  pub params: Vec<(Ident, Expr)>,
}
