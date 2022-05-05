use super::Rule;

/// Rule to constrain any type to a predicate
///
/// The value is always passed by its reference,
/// the closure need to accept that
///
/// # Example
/// ```
/// use type_rules::Validator;
/// use type_rules::rules::Eval;
///
/// const even_eval: for<'a> fn(&'a u32) -> bool = |v| v % 2 == 0;
///
/// #[derive(Validator)]
/// struct EvenInteger(
///     #[rule(Eval(even_eval, "Value need to be even"))]
///     u32
/// );
/// ```
pub struct Eval<'a, T>(pub T, pub &'a str);

impl<'a, T, U> Rule<U> for Eval<'a, T>
where
    T: Fn(&U) -> bool,
{
    fn check(&self, value: &U) -> Result<(), String> {
        match self.0(value) {
            true => Ok(()),
            false => Err(String::from(self.1)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rules::{Eval, Rule};
    use claim::{assert_err, assert_ok};

    const ERROR_MESSAGE: &str = "Value need to be even";
    const RULE: Eval<for<'a> fn(&'a u32) -> bool> = Eval(|v| v % 2 == 0, ERROR_MESSAGE);

    #[test]
    fn eval_ok() {
        assert_ok!(RULE.check(&2));
    }
    #[test]
    fn eval_err() {
        assert_err!(RULE.check(&1));
    }
    #[test]
    fn eval_good_error_message() {
        let res_error_message = RULE.check(&1).expect_err("Should be an Err");
        assert_eq!(res_error_message, ERROR_MESSAGE);
    }
}
