use super::Rule;

/// Rule to apply an Or condition on two rules.
///
/// In case of error on both rules, the first one is returned.
///
/// # Example
/// ```
/// use type_rules::prelude::*;
///
/// #[derive(Validator)]
/// struct NotZeroInteger(
///     #[rule(Or(MaxRange(-1), MinRange(1)))]
///     i32
/// );
/// ```
pub struct Or<T, U>(pub T, pub U);

impl<T, U, F> Rule<F> for Or<T, U>
where
    T: Rule<F>,
    U: Rule<F>,
{
    fn check(&self, value: &F) -> Result<(), String> {
        let first_res = self.0.check(value);
        let second_res = self.1.check(value);

        if first_res.is_ok() || second_res.is_ok() {
            Ok(())
        } else {
            first_res
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use claim::assert_ok;

    const RULE: Or<MaxRange<i32>, MinRange<i32>> = Or(MaxRange(-1), MinRange(1));

    #[test]
    fn or_0_ok() {
        assert_ok!(RULE.check(&-1));
    }

    #[test]
    fn or_1_ok() {
        assert_ok!(RULE.check(&1));
    }

    #[test]
    fn or_err() {
        let res_error_message = RULE.check(&0).expect_err("Should be an Err");

        assert_eq!(res_error_message, "Value is too high")
    }
}
