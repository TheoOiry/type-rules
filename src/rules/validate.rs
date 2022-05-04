use super::check_permissively_option;
use super::Rule;
use crate::Validator;

/// Rule to check the rules of the inner type
///
/// Works with [`Option`], just return `Ok(())` if it's [`None`]
///
/// # Example
/// ```
/// use type_rules::rules::{MaxLength, MinLength, Validate, MinMaxLength};
/// use type_rules::Validator;
///
/// #[derive(Validator)]
/// struct Password(#[rule(MinLength(8))] String);
///
/// #[derive(Validator)]
/// struct User {
///     username: String,
///     #[rule(Validate())]
///     password: Password,
/// };
/// ```
pub struct Validate();

impl<T: Validator> Rule<T> for Validate {
    fn check(&self, value: &T) -> Result<(), String> {
        value.check_validity()
    }
}

impl<T: Validator> Rule<Option<T>> for Validate {
    fn check(&self, value: &Option<T>) -> Result<(), String> {
        check_permissively_option(self, value)
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
        assert_ok!(Validate().check(&StringWrapper(String::from("a"))));
    }
    #[test]
    fn validate_err() {
        assert_err!(Validate().check(&StringWrapper(String::from("aaa"))));
    }
}
