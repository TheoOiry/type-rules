use super::Rule;

/// Rule to constrain an iterable collection (with reference)
/// to valid that any element of the collection valid the specified rule
///
/// # Example
/// ```
/// use type_rules::prelude::*;
///
/// #[derive(Validator)]
/// struct VecNotEmptyString(
///     #[rule(Any(MinLength(10), "At least one element need to be of length >= 10"))]
///     Vec<String>
/// );
/// ```
pub struct Any<'a, T>(pub T, pub &'a str);

impl<'a, T, U> Rule<U> for Any<'a, T>
where
    U: IntoIterator,
    for<'b> &'b U: IntoIterator<Item = &'b U::Item>,
    T: Rule<<U as IntoIterator>::Item>,
{
    fn check(&self, value: &U) -> Result<(), String> {
        match value
            .into_iter()
            .any(|v: &<U as IntoIterator>::Item| self.0.check(v).is_ok())
        {
            true => Ok(()),
            false => Err(String::from(self.1)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use claim::{assert_err, assert_ok};

    const ERROR_MESSAGE: &str = "At least one element need to be >= 1";
    const RULE: Any<MinRange<i32>> = Any(MinRange(1), ERROR_MESSAGE);

    #[test]
    fn any_ok() {
        assert_ok!(RULE.check(&vec![0, 1]));
    }
    #[test]
    fn any_err() {
        assert_err!(RULE.check(&vec![0, 0]));
    }
    #[test]
    fn any_good_error_message() {
        let res_error_message = RULE.check(&vec![0, 0]).expect_err("Should be an Err");
        assert_eq!(res_error_message, ERROR_MESSAGE);
    }
}
