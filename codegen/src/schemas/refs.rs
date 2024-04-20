use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Reference {
  #[serde(rename = "$ref")]
  pub ref_: String,
}

impl Reference {
  pub fn get_reference_id(&self) -> String {
    self
      .ref_
      .split('/')
      .last()
      .expect("Reference ID not found")
      .to_string()
  }
}
