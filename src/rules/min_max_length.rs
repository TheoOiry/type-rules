use super::Rule;
use super::{check_permissively_option, check_permissively_ref_option};

/// Rule to constraint the **minimum** and **maximum**
/// length of a [`String`] or `&str`
///
/// Works with [`Option`], just return `Ok(())` if it's [`None`]
///
/// # Example
/// ```
/// use type_rules::Validator;
/// use type_rules::rules::MinMaxLength;
///
/// #[derive(Validator)]
/// struct NewUser {
///     #[rule(MinMaxLength(3, 50))]
///     username: String,
///     #[rule(MinMaxLength(8, 100))]
///     password: String,
/// }
/// ```
pub struct MinMaxLength(pub usize, pub usize);

/// Rule to constraint the **minimum**
/// length of a [`String`] or `&str`
///
/// Works with [`Option`], just return `Ok(())` if it's [`None`]
///
/// # Example
/// ```
/// use type_rules::Validator;
/// use type_rules::rules::MinLength;
///
/// #[derive(Validator)]
/// struct Password(#[rule(MinLength(8))] String);
/// ```
pub struct MinLength(pub usize);

/// Rule to constraint the **maximum**
/// length of a [`String`] or `&str`
///
/// Works with [`Option`], just return `Ok(())` if it's [`None`]
///
/// # Example
/// ```
/// use type_rules::Validator;
/// use type_rules::rules::MaxLength;
///
/// #[derive(Validator)]
/// struct Payload(#[rule(MaxLength(200))] String);
/// ```
pub struct MaxLength(pub usize);

impl Rule<String> for MinMaxLength {
    fn check(&self, value: &String) -> Result<(), String> {
        check_value_too_short(value.len(), self.0)?;
        check_value_too_long(value.len(), self.1)
    }
}

impl Rule<str> for MinMaxLength {
    fn check(&self, value: &str) -> Result<(), String> {
        check_value_too_short(value.len(), self.0)?;
        check_value_too_long(value.len(), self.1)
    }
}

impl Rule<String> for MaxLength {
    fn check(&self, value: &String) -> Result<(), String> {
        check_value_too_long(value.len(), self.0)
    }
}

impl Rule<str> for MaxLength {
    fn check(&self, value: &str) -> Result<(), String> {
        check_value_too_long(value.len(), self.0)
    }
}

impl Rule<String> for MinLength {
    fn check(&self, value: &String) -> Result<(), String> {
        check_value_too_short(value.len(), self.0)
    }
}

impl Rule<str> for MinLength {
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

impl Rule<Option<String>> for MinMaxLength {
    fn check(&self, value: &Option<String>) -> Result<(), String> {
        check_permissively_option(self, value)
    }
}

impl Rule<Option<&str>> for MinMaxLength {
    fn check(&self, value: &Option<&str>) -> Result<(), String> {
        check_permissively_ref_option(self, value)
    }
}

impl Rule<Option<String>> for MinLength {
    fn check(&self, value: &Option<String>) -> Result<(), String> {
        check_permissively_option(self, value)
    }
}

impl Rule<Option<&str>> for MinLength {
    fn check(&self, value: &Option<&str>) -> Result<(), String> {
        check_permissively_ref_option(self, value)
    }
}

impl Rule<Option<String>> for MaxLength {
    fn check(&self, value: &Option<String>) -> Result<(), String> {
        check_permissively_option(self, value)
    }
}

impl Rule<Option<&str>> for MaxLength {
    fn check(&self, value: &Option<&str>) -> Result<(), String> {
        check_permissively_ref_option(self, value)
    }
}

#[cfg(test)]
mod tests {
    use crate::rules::{MaxLength, MinLength, MinMaxLength, Rule};
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
