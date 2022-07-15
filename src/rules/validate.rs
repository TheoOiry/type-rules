use super::Rule;
use crate::Validator;

/// Rule to check the rules of the inner type
///
/// # Example
/// ```
/// use type_rules::prelude::*;
///
/// #[derive(Validator)]
/// struct Password(#[rule(MinLength(8))] String);
///
/// #[derive(Validator)]
/// struct User {
///     username: String,
///     #[rule(Validate)]
///     password: Password,
/// };
/// ```
pub struct Validate;

impl<T: Validator> Rule<T> for Validate {
    fn check(&self, value: &T) -> Result<(), String> {
        value.check_validity()
    }
}

#[cfg(test)]
mod tests {
    use crate::rules::{MaxLength, Rule, Validate};
    use crate::Validator;
    use claim::{assert_err, assert_ok};

    struct StringWrapper(String);

    impl Validator for StringWrapper {
        fn check_validity(&self) -> Result<(), String> {
            MaxLength(2).check(&self.0)
        }
    }

    #[test]
    fn validate_ok() {
        assert_ok!(Validate.check(&StringWrapper(String::from("a"))));
    }
    #[test]
    fn validate_err() {
        assert_err!(Validate.check(&StringWrapper(String::from("aaa"))));
    }
}
