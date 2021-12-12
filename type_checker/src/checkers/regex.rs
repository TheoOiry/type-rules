use super::Checker;

use regex::Regex;

pub struct RegEx<'a>(pub &'a str);

impl<'a, T: AsRef<str> + ?Sized> Checker<T> for RegEx<'a> {
    fn check(&self, value: &T) -> Result<(), String> {
        let regex = Regex::new(self.0).expect("Invalid Regex");
        if regex.is_match(value.as_ref()) {
            return Ok(());
        }
        Err(String::from("The regex does not match"))
    }
}

#[cfg(test)]
mod tests {
    use claim::{ assert_ok, assert_err };
    use crate::checkers::{ Checker, RegEx };

    #[test]
    fn regex_ok() {
        assert_ok!(RegEx(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").check("theo.oiry@yahoo.fr"));
    }
    #[test]
    fn regex_err() {
        assert_err!(RegEx(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").check("theo.oiryyahoo.fr"));
    }
}