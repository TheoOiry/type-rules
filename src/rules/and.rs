use super::Rule;

/// Rule to ensure that 2 other rules are [`Ok`].
///
/// The `rule` attribute accepts multiple rules and has the same behavior
/// but the [`And`] rule is useful for rules that accept a single rule such
/// as [`All`] and [`Opt`].
///
/// In case of error on both rules, the first one is returned.
///
/// # Example
/// ```
/// use type_rules::prelude::*;
///
/// #[derive(Validator)]
/// struct OptionalMail(
///     #[rule(Opt(And(MaxLength(1000), RegEx(r"^\S+@\S+\.\S+"))))]
///     Option<String>
/// );
/// ```
pub struct And<T, U>(pub T, pub U);

impl<T, U, F> Rule<F> for And<T, U>
where
    T: Rule<F>,
    U: Rule<F>,
{
    fn check(&self, value: &F) -> Result<(), String> {
        self.0.check(value)?;
        self.1.check(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use claim::assert_ok;

    const RULE: Opt<And<MaxLength, RegEx>> = Opt(And(MaxLength(20), RegEx(r"^\S+@\S+\.\S+")));

    #[test]
    fn and_ok() {
        assert_ok!(RULE.check(&Some("example@example.fr")));
    }

    #[test]
    fn and_0_err() {
        let val = Some("too.long.example@too.long.example.fr");

        let res_error_message = RULE.check(&val).expect_err("Should be an Err");

        assert_eq!(res_error_message, "Value is too long")
    }

    #[test]
    fn and_1_err() {
        let val = Some("example.example.fr");

        let res_error_message = RULE.check(&val).expect_err("Should be an Err");

        assert_eq!(res_error_message, "The regex does not match")
    }

    #[test]
    fn and_0_1_err() {
        let val = Some("too.long.example.too.long.example.fr");

        let res_error_message = RULE.check(&val).expect_err("Should be an Err");

        assert_eq!(res_error_message, "Value is too long")
    }
}
