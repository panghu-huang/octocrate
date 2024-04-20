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
  let parsed_api_descrption = codegen.parse();

  let dir = std::path::PathBuf::from("octocrate/src");

  codegen.write(parsed_api_descrption, &dir);

  println!("✨ Done in {}", HumanDuration(start_time.elapsed()));
}
