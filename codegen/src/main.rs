mod codegen;
mod common;
mod parser;
mod schemas;
mod structures;
mod writer;

use codegen::{Codegen, WriteOptions};
use indicatif::HumanDuration;
use std::{path::PathBuf, time::Instant};

fn main() {
  let codegen = Codegen::new();

  let start_time = Instant::now();
  let parsed_api_description = codegen.parse();

  let write_options = WriteOptions {
    apis_path: PathBuf::from("octocrate/src"),
    types_path: PathBuf::from("types/src"),
    webhooks_path: PathBuf::from("webhooks/src"),
  };

  codegen.write(&parsed_api_description, &write_options);

  println!("✨ Done in {}", HumanDuration(start_time.elapsed()));
}
