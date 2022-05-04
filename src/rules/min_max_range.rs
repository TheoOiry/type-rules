use super::check_permissively_option;
use super::Rule;

pub struct MinMaxRange<T: PartialOrd<T>>(pub T, pub T);

pub struct MinRange<T: PartialOrd<T>>(pub T);

pub struct MaxRange<T: PartialOrd<T>>(pub T);

impl<T: PartialOrd<T>> Rule<T> for MinMaxRange<T> {
    fn check(&self, value: &T) -> Result<(), String> {
        check_value_too_low(value, &self.0)?;
        check_value_too_high(value, &self.1)?;
        Ok(())
    }
}

impl<T: PartialOrd<T>> Rule<T> for MinRange<T> {
    fn check(&self, value: &T) -> Result<(), String> {
        check_value_too_low(value, &self.0)?;
        Ok(())
    }
}

impl<T: PartialOrd<T>> Rule<T> for MaxRange<T> {
    fn check(&self, value: &T) -> Result<(), String> {
        check_value_too_high(value, &self.0)?;
        Ok(())
    }
}

fn check_value_too_low<T: PartialOrd<T>>(value: &T, min_range: &T) -> Result<(), String> {
    if value < min_range {
        return Err(String::from("Value is too low"));
    }
    Ok(())
}

fn check_value_too_high<T: PartialOrd<T>>(value: &T, max_range: &T) -> Result<(), String> {
    if value > max_range {
        return Err(String::from("Value is too high"));
    }
    Ok(())
}

impl<T: PartialOrd<T>> Rule<Option<T>> for MinMaxRange<T> {
    fn check(&self, value: &Option<T>) -> Result<(), String> {
        check_permissively_option(self, value)
    }
}


impl<T: PartialOrd<T>> Rule<Option<T>> for MaxRange<T> {
    fn check(&self, value: &Option<T>) -> Result<(), String> {
        check_permissively_option(self, value)
    }
}

impl<T: PartialOrd<T>> Rule<Option<T>> for MinRange<T> {
    fn check(&self, value: &Option<T>) -> Result<(), String> {
        check_permissively_option(self, value)
    }
}

#[cfg(test)]
mod tests {
    use crate::rules::{Rule, MinRange, MaxRange, MinMaxRange};
    use claim::{assert_err, assert_ok};

    #[test]
    fn min_range_value_too_short() {
        assert_err!(MinRange(10).check(&9));
    }
    #[test]
    fn min_range_value_ok() {
        assert_ok!(MinRange(10).check(&10));
    }
    #[test]
    fn max_range_value_too_long() {
        assert_err!(MaxRange(10).check(&11));
    }
    #[test]
    fn max_range_value_ok() {
        assert_ok!(MaxRange(10).check(&10));
    }
    #[test]
    fn min_max_range_value_too_short() {
        assert_err!(MinMaxRange(10, 100).check(&5));
    }
    #[test]
    fn min_max_range_value_too_long() {
        assert_err!(MinMaxRange(10, 100).check(&101));
    }
    #[test]
    fn min_max_range_value_ok() {
        assert_ok!(MinMaxRange(10, 100).check(&50));
    }
}
