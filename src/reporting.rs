#![allow(dead_code)]

pub fn error(line: usize, message: &str) {
  report(line, "", message);
}

pub fn report(line: usize, location: &str, message: &str) {
  println!("[line {line}] Error {location}: {message}");
}
