#![doc = include_str!("../README.md")]

pub mod rules;

#[cfg(feature = "type-rules-derive")]
#[doc(hidden)]
pub use type_rules_derive::*;

#[doc(inline)]
pub use rules::Rule;

/// Check the validity of a type
///
/// By implementing `Validator` for a type, you define the
/// rules to check is validity
///
/// Can be derived with the `derive` feature
///
/// # Example
///
/// Basic usage:
///
/// ```should_panic
/// use type_rules::Validator;
/// use type_rules::rules::MinLength;
///
/// #[derive(Validator)]
/// struct NotEmptyString(#[rule(MinLength(1))] String);
///
/// let valid = NotEmptyString(String::from("Not empty"));
/// let not_valid = NotEmptyString(String::from(""));
///
/// valid.check_validity().unwrap(); // OK
/// not_valid.check_validity().unwrap(); // Value is too short
/// ```
pub trait Validator {
    fn check_validity(&self) -> Result<(), String>;
}
