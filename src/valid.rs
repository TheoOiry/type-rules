use crate::Validator;
use std::ops::Deref;

#[derive(Debug, Copy, Clone)]
pub struct Valid<T>(T);

impl<T: Validator> Valid<T> {
    #[allow(dead_code)]
    pub fn new(val: T) -> Result<Self, String> {
        val.check_validity()?;
        Ok(Valid(val))
    }
}

impl<T> Deref for Valid<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::valid::Valid;
    use crate::Validator;
    use claim::{assert_err, assert_ok};

    #[derive(Debug)]
    struct ValidTest(i32);

    impl Validator for ValidTest {
        fn check_validity(&self) -> Result<(), String> {
            match self.0.is_positive() {
                true => Ok(()),
                false => Err(String::from("Need to be positive")),
            }
        }
    }

    #[test]
    fn valid_ok() {
        assert_ok!(Valid::new(ValidTest(1)));
    }

    #[test]
    fn valid_err() {
        assert_err!(Valid::new(ValidTest(-1)));
    }
}
