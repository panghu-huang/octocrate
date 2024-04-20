use std::io::Write;
use std::process::{Command, Stdio};

/// Format the provided code.
pub fn format_code(module: impl AsRef<[u8]>) -> String {
  let mut command = Command::new("rustfmt")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .expect("Failed to start rustfmt, make sure it is installed.");

  // Get stdin handle of the child process
  if let Some(child_stdin) = command.stdin.as_mut() {
    child_stdin
      .write_all(module.as_ref())
      .expect("Failed to pass module to `rustfmt`");
    child_stdin.flush().unwrap();
  }

  let output = command.wait_with_output().unwrap();

  if output.status.success() {
    String::from_utf8(output.stdout).unwrap()
  } else {
    panic!("Could not format the provided content");
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_format_code() {
    let module = r#"fn a   ()    {}



const  B:   &'static str  =   "testing";
    "#;
    let expected = "fn a() {}\n\nconst B: &'static str = \"testing\";\n";

    assert_eq!(format_code(module), expected);
  }
}
