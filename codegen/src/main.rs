mod codegen;
mod common;
mod parser;
mod schemas;
mod structures;
mod writer;

use codegen::Codegen;
use indicatif::HumanDuration;
use std::time::Instant;

fn main() {
  let codegen = Codegen::new();

  let start_time = Instant::now();
  let parsed_api_description = codegen.parse();

  let apis_dir = std::path::PathBuf::from("octocrate/src");
  let types_dir = std::path::PathBuf::from("types/src");

  codegen.write_apis(parsed_api_description.clone(), &apis_dir);
  codegen.write_types(parsed_api_description, &types_dir);

  println!("✨ Done in {}", HumanDuration(start_time.elapsed()));
}
