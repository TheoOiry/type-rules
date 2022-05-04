use super::Rule;
use super::{check_permissively_option, check_permissively_ref_option};
use regex_helper::Regex;

/// Rule to constraint a [`String`] or `&str` to match a Regex
///
/// You need the `regex` feature to use it
///
/// Works with [`Option`], just return `Ok(())` if it's [`None`]
///
/// # Example
/// ```
/// use type_rules::Validator;
/// use type_rules::rules::RegEx;
///
/// #[derive(Validator)]
/// struct Mail(#[rule(RegEx(r"^\S+@\S+\.\S+"))] String);
/// ```
pub struct RegEx<'a>(pub &'a str);

impl<'a> Rule<String> for RegEx<'a> {
    fn check(&self, value: &String) -> Result<(), String> {
        check(self.0, value)
    }
}

impl<'a> Rule<str> for RegEx<'a> {
    fn check(&self, value: &str) -> Result<(), String> {
        check(self.0, value)
    }
}

impl<'a> Rule<Option<String>> for RegEx<'a> {
    fn check(&self, value: &Option<String>) -> Result<(), String> {
        check_permissively_option(self, value)
    }
}

impl<'a> Rule<Option<&str>> for RegEx<'a> {
    fn check(&self, value: &Option<&str>) -> Result<(), String> {
        check_permissively_ref_option(self, value)
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
