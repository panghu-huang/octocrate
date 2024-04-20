mod api;
mod format;
mod types;

pub use api::*;
pub use format::*;
use std::path::PathBuf;
pub use types::*;

pub trait File {
  fn file_name(&self) -> String;
  fn write(&self, path: &PathBuf);
}

pub struct Writer<'a> {
  pub path: &'a PathBuf,
  pub files: Vec<Box<dyn File>>,
}

pub struct Directory {
  pub name: String,
  pub files: Vec<Box<dyn File>>,
}

impl<'a> Writer<'a> {
  pub fn new(path: &'a PathBuf) -> Writer {
    Writer {
      path,
      files: Vec::new(),
    }
  }

  pub fn add_file<T>(&mut self, file: T)
  where
    T: File + 'static,
  {
    self.files.push(Box::new(file));
  }

  pub fn write(&self) {
    for file in &self.files {
      file.write(self.path);
    }
  }
}

impl Directory {
  pub fn new(name: &str) -> Directory {
    Directory {
      name: name.to_string(),
      files: Vec::new(),
    }
  }

  pub fn add_file<T>(&mut self, file: T)
  where
    T: File + 'static,
  {
    self.files.push(Box::new(file));
  }
}

impl File for Directory {
  fn file_name(&self) -> String {
    self.name.clone()
  }

  fn write(&self, path: &PathBuf) {
    let dir_path = path.join(self.file_name());

    if dir_path.exists() {
      std::fs::remove_dir_all(&dir_path).expect("Unable to remove directory");
    }

    std::fs::create_dir_all(&dir_path).expect("Unable to create directory");

    for file in &self.files {
      file.write(&dir_path);
    }
  }
}
