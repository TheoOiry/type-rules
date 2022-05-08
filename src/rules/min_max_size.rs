use super::Rule;

/// Rule to constraint the **minimum** and **maximum**
/// size of a [`Vec<T>`]
///
/// # Example
/// ```
/// use type_rules::prelude::*;
///
/// #[derive(Validator)]
/// struct Car {
///     #[rule(MinMaxSize(4, 4))]
///     wheels_diameter: Vec<f32>
/// };
/// ```
pub struct MinMaxSize(pub usize, pub usize);

/// Rule to constraint the **minimum**
/// size of a [`Vec<T>`]
///
/// # Example
/// ```
/// use type_rules::prelude::*;
///
/// #[derive(Validator)]
/// struct NonEmptyVec<T>(#[rule(MinSize(1))] Vec<T>);
/// ```
pub struct MinSize(pub usize);

/// Rule to constraint the **maximum**
/// size of a [`Vec<T>`]
///
/// # Example
/// ```
/// use type_rules::prelude::*;
///
/// #[derive(Validator)]
/// struct FollowedCategories(#[rule(MaxSize(100))] Vec<String>);
/// ```
pub struct MaxSize(pub usize);

impl<T> Rule<Vec<T>> for MinMaxSize {
    fn check(&self, value: &Vec<T>) -> Result<(), String> {
        let size = value.len();
        check_value_too_short(size, self.0)?;
        check_value_too_long(size, self.1)?;
        Ok(())
    }
}

impl<T> Rule<Vec<T>> for MinSize {
    fn check(&self, value: &Vec<T>) -> Result<(), String> {
        check_value_too_short(value.len(), self.0)?;
        Ok(())
    }
}

impl<T> Rule<Vec<T>> for MaxSize {
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

#[cfg(test)]
mod tests {
    use crate::rules::{MaxSize, MinMaxSize, MinSize, Rule};
    use claim::{assert_err, assert_ok};

    #[test]
    fn min_size_value_too_short() {
        assert_err!(MinSize(1).check(&Vec::<i32>::new()));
    }
    #[test]
    fn min_size_value_ok() {
        assert_ok!(MinSize(1).check(&vec![1, 2]));
    }
    #[test]
    fn max_size_value_too_long() {
        assert_err!(MaxSize(1).check(&vec![1, 2]));
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
        assert_err!(MinMaxSize(1, 2).check(&vec![1, 2, 3]));
    }
    #[test]
    fn min_max_size_value_ok() {
        assert_ok!(MinMaxSize(1, 2).check(&vec![1]));
    }
}
