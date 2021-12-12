mod min_max_length;
mod min_max_range;
mod min_max_size;
mod validate;
mod regex;

pub use min_max_length::{ MinLength, MaxLength, MinMaxLength };
pub use min_max_range::{ MinRange, MaxRange, MinMaxRange };
pub use min_max_size::{ MinSize, MaxSize, MinMaxSize };
pub use validate::Validate;
pub use self::regex::RegEx;

pub trait Checker<T: ?Sized> {
    fn check(&self, value: &T) -> Result<(), String>;
}
