use super::check_permissively_option;
use crate::Validator;
use super::Rule;

/// Rule to check the rules of the inner type
///
/// Works with [`Option`], just return `Ok(())` if it's [`None`]
///
/// # Example
/// ```
/// use type_rules::rules::{MaxLength, RegEx, Validate, MinMaxLength};
/// use type_rules::Validator;
///
/// #[derive(Validator)]
/// struct EmailWrapper(#[rule(MaxLength(100), RegEx(r"^\S+@\S+\.\S+"))] String);
///
/// #[derive(Validator)]
/// struct User {
///     #[rule(Validate())]
///     email: EmailWrapper,
///     #[rule(MinMaxLength(8, 50))]
///     password: String,
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
    use claim::{ assert_ok, assert_err };
    use crate::rules::{Rule, MaxLength, Validate};
    use crate::Validator;

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