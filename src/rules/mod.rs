mod min_max_length;
mod min_max_range;
mod min_max_size;
mod validate;
mod regex;

pub use self::min_max_length::*;
pub use self::min_max_range::*;
pub use self::min_max_size::*;
pub use self::validate::*;
pub use self::regex::*;

pub trait Rule<T: ?Sized> {
    fn check(&self, value: &T) -> Result<(), String>;
}

fn check_permissively_option<U: Rule<T>, T>(rule: &U, value: &Option<T>) -> Result<(), String> {
    match value {
        Some(val) => rule.check(val),
        None => Ok(())
    }
}

fn check_permissively_ref_option<U: Rule<T>, T: ?Sized>(rule: &U, value: &Option<&T>) -> Result<(), String> {
    match value {
        Some(val) => rule.check(*val),
        None => Ok(())
    }
}