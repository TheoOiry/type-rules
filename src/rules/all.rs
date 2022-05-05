use super::Rule;

/// Rule to constrain a collection that is iterable (with reference)
/// to valid the specified rule and specify an error message
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
