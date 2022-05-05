use super::Rule;

/// Rule to apply another rule to inner value of an [`Option`]
///
/// In case of a [`None`] value, it just return `Ok(())`
///
/// # Example
/// ```
/// use type_rules::Validator;
/// use type_rules::rules::{Opt, MinMaxRange};
///
/// #[derive(Validator)]
/// struct Parameters {
///     #[rule(Opt(MinMaxRange(1, 4)))]
///     optional_parameter: Option<u32>,
/// }
///
/// let param = Parameters { optional_parameter: Some(1) };
/// assert!(param.check_validity().is_ok());
///
/// let param = Parameters { optional_parameter: None };
/// assert!(param.check_validity().is_ok());
///
/// let param = Parameters { optional_parameter: Some(5) };
/// assert!(param.check_validity().is_err());
/// ```
pub struct Opt<T>(pub T);

impl<T, U: Rule<T>> Rule<Option<T>> for Opt<U> {
    fn check(&self, value: &Option<T>) -> Result<(), String> {
        match value {
            Some(val) => self.0.check(val),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rules::{MinRange, Opt, Rule};
    use claim::{assert_err, assert_ok};

    const RULE: Opt<MinRange<i32>> = Opt(MinRange(1));

    #[test]
    fn opt_ok_with_some() {
        assert_ok!(RULE.check(&Some(1)));
    }
    #[test]
    fn opt_ok_with_none() {
        assert_ok!(RULE.check(&None));
    }
    #[test]
    fn opt_err() {
        assert_err!(RULE.check(&Some(0)));
    }
}
