use crate::checkers::check_permissively_option;
use super::Checker;

pub struct MinMaxSize(pub usize, pub usize);

pub struct MinSize(pub usize);

pub struct MaxSize(pub usize);

impl<T> Checker<Vec<T>> for MinMaxSize {
    fn check(&self, value: &Vec<T>) -> Result<(), String> {
        let size = value.len();
        check_value_too_short(size, self.0)?;
        check_value_too_long(size, self.1)?;
        Ok(())
    }
}

impl<T> Checker<Vec<T>> for MinSize {
    fn check(&self, value: &Vec<T>) -> Result<(), String> {
        check_value_too_short(value.len(), self.0)?;
        Ok(())
    }
}

impl<T> Checker<Vec<T>> for MaxSize {
    fn check(&self, value: &Vec<T>) -> Result<(), String> {
        check_value_too_long(value.len(), self.0)?;
        Ok(())
    }
}

fn check_value_too_short(length: usize, min_size: usize) -> Result<(), String> {
    if length < min_size {
        return Err(String::from("Collection is too short"));
    }
    Ok(())
}

fn check_value_too_long(length: usize, max_size: usize) -> Result<(), String> {
    if length > max_size {
        return Err(String::from("Collection is too long"));
    }
    Ok(())
}

impl<T> Checker<Option<Vec<T>>> for MinMaxSize {
    fn check(&self, value: &Option<Vec<T>>) -> Result<(), String> {
        check_permissively_option(self, value)
    }
}

impl<T> Checker<Option<Vec<T>>> for MaxSize {
    fn check(&self, value: &Option<Vec<T>>) -> Result<(), String> {
        check_permissively_option(self, value)
    }
}

impl<T> Checker<Option<Vec<T>>> for MinSize {
    fn check(&self, value: &Option<Vec<T>>) -> Result<(), String> {
        check_permissively_option(self, value)
    }
}

#[cfg(test)]
mod tests {
    use crate::checkers::{Checker, MinSize, MaxSize, MinMaxSize};
    use claim::{assert_err, assert_ok};

    #[test]
    fn min_size_value_too_short() {
        assert_err!(MinSize(1).check(&Vec::<i32>::new()));
    }
    #[test]
    fn min_size_value_ok() {
        assert_ok!(MinSize(1).check(&vec![1,2]));
    }
    #[test]
    fn max_size_value_too_long() {
        assert_err!(MaxSize(1).check(&vec![1,2]));
    }
    #[test]
    fn max_size_value_ok() {
        assert_ok!(MaxSize(1).check(&vec![1]));
    }
    #[test]
    fn min_max_size_value_too_short() {
        assert_err!(MinMaxSize(1, 2).check(&Vec::<i32>::new()));
    }
    #[test]
    fn min_max_size_value_too_long() {
        assert_err!(MinMaxSize(1, 2).check(&vec![1,2,3]));
    }
    #[test]
    fn min_max_size_value_ok() {
        assert_ok!(MinMaxSize(1, 2).check(&vec![1]));
    }
}
