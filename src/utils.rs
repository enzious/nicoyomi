use regex::Regex;

pub fn safe_str(input: &str) -> String {
  let safe = Regex::new(r#"[^a-zA-Z\d:]+"#)
    .unwrap()
    .replace_all(input, "-");

  Regex::new(r#"(^-|-$)"#)
    .unwrap()
    .replace_all(&safe, "")
    .into_owned()
}
