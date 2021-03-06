use super::Rule;
use regex::Regex;

/// Rule to constraint any type that implements [`AsRef<str>`] such
/// as [`String`] or `&str` to match a Regex
///
/// You need the `regex` feature to use it
///
/// # Example
/// ```
/// use type_rules::prelude::*;
///
/// #[derive(Validator)]
/// struct Mail(#[rule(RegEx(r"^\S+@\S+\.\S+"))] String);
/// ```
pub struct RegEx<'a>(pub &'a str);

impl<'a, T: AsRef<str> + ?Sized> Rule<T> for RegEx<'a> {
    fn check(&self, value: &T) -> Result<(), String> {
        check(self.0, value.as_ref())
    }
}

fn check(regex: &str, value: &str) -> Result<(), String> {
    let regex = Regex::new(regex).expect("Invalid Regex");
    if regex.is_match(value) {
        return Ok(());
    }
    Err(String::from("The regex does not match"))
}

#[cfg(test)]
mod tests {
    use crate::rules::{RegEx, Rule};
    use claim::{assert_err, assert_ok};

    #[test]
    fn regex_ok() {
        assert_ok!(RegEx(r"^\S+@\S+\.\S+").check("example@example.fr"));
    }
    #[test]
    fn regex_err() {
        assert_err!(RegEx(r"^\S+@\S+\.\S+").check("exampleexample.fr"));
    }
}
