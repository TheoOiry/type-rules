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

pub trait Checker<T: ?Sized> {
    fn check(&self, value: &T) -> Result<(), String>;
}
