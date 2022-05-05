use super::Rule;

/// Rule to constrain an iterable collection (with reference)
/// to valid the specified rule and an error message
///
/// # Example
/// ```
/// use type_rules::Validator;
/// use type_rules::rules::{All, MinLength};
///
/// #[derive(Validator)]
/// struct VecNotEmptyString(
///     #[rule(All(MinLength(1), "You can't use empty string"))]
///     Vec<String>
/// );
/// ```
pub struct All<'a, T>(pub T, pub &'a str);

impl<'a, T, U> Rule<U> for All<'a, T>
where
    U: IntoIterator,
    for<'b> &'b U: IntoIterator<Item = &'b U::Item>,
    T: Rule<<U as IntoIterator>::Item>,
{
    fn check(&self, value: &U) -> Result<(), String> {
        match value
            .into_iter()
            .all(|v: &<U as IntoIterator>::Item| self.0.check(v).is_ok())
        {
            true => Ok(()),
            false => Err(String::from(self.1)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rules::{All, MinRange, Rule};
    use claim::{assert_err, assert_ok};

    const ERROR_MESSAGE: &str = "Value need to be >= 1";
    const RULE: All<MinRange<i32>> = All(MinRange(1), ERROR_MESSAGE);

    #[test]
    fn all_ok() {
        assert_ok!(RULE.check(&vec![1, 1]));
    }
    #[test]
    fn all_err() {
        assert_err!(RULE.check(&vec![1, 0]));
    }
    #[test]
    fn all_good_error_message() {
        let res_error_message = RULE.check(&vec![1, 0]).expect_err("Should be an Err");
        assert_eq!(res_error_message, ERROR_MESSAGE);
    }
}
