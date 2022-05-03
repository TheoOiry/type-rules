use crate::checkers::check_permissively_option;
use crate::Validator;
use super::Checker;

pub struct Validate();

impl<T: Validator> Checker<T> for Validate {
    fn check(&self, value: &T) -> Result<(), String> {
        value.check_validity()
    }
}

impl<T: Validator> Checker<Option<T>> for Validate {
    fn check(&self, value: &Option<T>) -> Result<(), String> {
        check_permissively_option(self, value)
    }
}

#[cfg(test)]
mod tests {
    use claim::{ assert_ok, assert_err };
    use crate::checkers::{Checker, MaxLength, Validate};
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