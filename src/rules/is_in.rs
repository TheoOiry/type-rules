use super::Rule;

/// Rule to constrain a type to be `in` a collection
///
/// # Example
/// ```
/// use type_rules::prelude::*;
///
/// #[derive(Validator)]
/// struct Fruit(#[rule(In(["apple", "banana", "orange", "pear"], "Value need to be a fruit"))] String);
/// ```
pub struct In<'a, T>(pub T, pub &'a str);

impl<'a, T, U> Rule<U> for In<'a, T>
where
    T: IntoIterator,
    for<'b> &'b T: IntoIterator<Item = &'b T::Item>,
    U: PartialEq<T::Item>,
{
    fn check(&self, value: &U) -> Result<(), String> {
        if (&self.0).into_iter().any(|v| value == v) {
            Ok(())
        } else {
            Err(String::from(self.1))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use claim::assert_ok;

    const ERROR_MESSAGE: &str = "Value need to be a fruit";
    const RULE: In<[&str; 4]> = In(["apple", "banana", "orange", "pear"], ERROR_MESSAGE);

    #[test]
    fn in_ok() {
        assert_ok!(RULE.check(&"banana"));
    }
    #[test]
    fn in_err() {
        let res_error_message = RULE.check(&"sandwich").expect_err("Should be an Err");
        assert_eq!(res_error_message, ERROR_MESSAGE);
    }
}
