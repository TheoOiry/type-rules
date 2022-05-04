use regex_helper::Regex;
use super::{check_permissively_option, check_permissively_ref_option};
use super::Rule;

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
    use claim::{ assert_ok, assert_err };
    use crate::rules::{Rule, RegEx };

    #[test]
    fn regex_ok() {
        assert_ok!(RegEx(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").check("theo.oiry@yahoo.fr"));
    }
    #[test]
    fn regex_err() {
        assert_err!(RegEx(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").check("theo.oiryyahoo.fr"));
    }
}