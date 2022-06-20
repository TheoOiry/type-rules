mod all;
mod and;
mod any;
mod eval;
mod is_in;
mod min_max_length;
mod min_max_range;
mod min_max_size;
mod opt;
mod or;
#[cfg(feature = "regex")]
#[cfg_attr(docsrs, doc(cfg(feature = "regex")))]
mod regex;
mod validate;

pub use self::all::*;
pub use self::and::*;
pub use self::any::*;
pub use self::eval::*;
pub use self::is_in::*;
pub use self::min_max_length::*;
pub use self::min_max_range::*;
pub use self::min_max_size::*;
pub use self::opt::*;
pub use self::or::*;
pub use self::validate::*;

#[cfg(feature = "regex")]
#[cfg_attr(docsrs, doc(cfg(feature = "regex")))]
pub use self::regex::*;

/// Define a rule for a type
///
/// By implementing `Rule` for a type you define how
/// it will be used to constraint a type `T`
///
/// # Example
///
/// ```
/// use type_rules::prelude::*;
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
