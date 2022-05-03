use crate::checkers::{check_permissively_option, check_permissively_ref_option};
use super::Checker;

pub struct MinMaxLength(pub usize, pub usize);

pub struct MinLength(pub usize);

pub struct MaxLength(pub usize);

impl Checker<String> for MinMaxLength {
    fn check(&self, value: &String) -> Result<(), String> {
        check_value_too_short(value.len(), self.0)?;
        check_value_too_long(value.len(), self.1)
    }
}

impl Checker<str> for MinMaxLength {
    fn check(&self, value: &str) -> Result<(), String> {
        check_value_too_short(value.len(), self.0)?;
        check_value_too_long(value.len(), self.1)
    }
}

impl Checker<String> for MaxLength {
    fn check(&self, value: &String) -> Result<(), String> {
        check_value_too_long(value.len(), self.0)
    }
}

impl Checker<str> for MaxLength {
    fn check(&self, value: &str) -> Result<(), String> {
        check_value_too_long(value.len(), self.0)
    }
}

impl Checker<String> for MinLength {
    fn check(&self, value: &String) -> Result<(), String> {
        check_value_too_short(value.len(), self.0)
    }
}

impl Checker<str> for MinLength {
    fn check(&self, value: &str) -> Result<(), String> {
        check_value_too_short(value.len(), self.0)
    }
}

fn check_value_too_short(length: usize, min_length: usize) -> Result<(), String> {
    if min_length > length {
        return Err(String::from("Value is too short"));
    }
    Ok(())
}

fn check_value_too_long(length: usize, max_length: usize) -> Result<(), String> {
    if max_length < length {
        return Err(String::from("Value is too long"));
    }
    Ok(())
}

impl Checker<Option<String>> for MinMaxLength {
    fn check(&self, value: &Option<String>) -> Result<(), String> {
        check_permissively_option(self, value)
    }
}

impl Checker<Option<&str>> for MinMaxLength {
    fn check(&self, value: &Option<&str>) -> Result<(), String> {
        check_permissively_ref_option(self, value)
    }
}

impl Checker<Option<String>> for MinLength {
    fn check(&self, value: &Option<String>) -> Result<(), String> {
        check_permissively_option(self, value)
    }
}

impl Checker<Option<&str>> for MinLength {
    fn check(&self, value: &Option<&str>) -> Result<(), String> {
        check_permissively_ref_option(self, value)
    }
}

impl Checker<Option<String>> for MaxLength {
    fn check(&self, value: &Option<String>) -> Result<(), String> {
        check_permissively_option(self, value)
    }
}

impl Checker<Option<&str>> for MaxLength {
    fn check(&self, value: &Option<&str>) -> Result<(), String> {
        check_permissively_ref_option(self, value)
    }
}

#[cfg(test)]
mod tests {
    use crate::checkers::{Checker, MinLength, MaxLength, MinMaxLength};
    use claim::{assert_err, assert_ok};

    #[test]
    fn min_length_value_too_short() {
        assert_err!(MinLength(1).check(""));
    }
    #[test]
    fn min_length_value_ok() {
        assert_ok!(MinLength(1).check("a"));
    }
    #[test]
    fn max_length_value_too_long() {
        assert_err!(MaxLength(1).check("aa"));
    }
    #[test]
    fn max_length_value_ok() {
        assert_ok!(MaxLength(1).check("a"));
    }
    #[test]
    fn min_max_length_value_too_short() {
        assert_err!(MinMaxLength(1, 10).check(""));
    }
    #[test]
    fn min_max_length_value_too_long() {
        assert_err!(MinMaxLength(0, 1).check("aa"));
    }
    #[test]
    fn min_max_length_value_ok() {
        assert_ok!(MinMaxLength(0, 1).check("a"));
    }
}
