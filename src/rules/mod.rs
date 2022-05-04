mod min_max_length;
mod min_max_range;
mod min_max_size;
mod validate;

#[cfg(feature = "regex_helper")]
mod regex;

pub use self::min_max_length::*;
pub use self::min_max_range::*;
pub use self::min_max_size::*;
pub use self::validate::*;

#[cfg(feature = "regex_helper")]
pub use self::regex::*;

/// Define a rule for a type
///
/// By implementing `Rule` for a type you define how
/// it will be used to constraint a type `T`
///
/// # Example
///
/// ```
/// use type_rules::{Validator, Rule};
///
/// struct IsEven();
///
/// impl Rule<i32> for IsEven {
///     fn check(&self, value: &i32) -> Result<(), String> {
///         if value % 2 == 0 {
///             Ok(())
///         } else {
///             Err("Value is not even".into())
///         }
///     }
/// }
///
/// #[derive(Validator)]
/// struct MyInteger(#[rule(IsEven())] i32);
/// ```
pub trait Rule<T: ?Sized> {
    fn check(&self, value: &T) -> Result<(), String>;
}

fn check_permissively_option<U: Rule<T>, T>(rule: &U, value: &Option<T>) -> Result<(), String> {
    match value {
        Some(val) => rule.check(val),
        None => Ok(()),
    }
}

fn check_permissively_ref_option<U: Rule<T>, T: ?Sized>(
    rule: &U,
    value: &Option<&T>,
) -> Result<(), String> {
    match value {
        Some(val) => rule.check(*val),
        None => Ok(()),
    }
}
