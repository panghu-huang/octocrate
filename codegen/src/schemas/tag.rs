use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Tag {
  pub name: String,
  pub description: String,
}
