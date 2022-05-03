pub mod checkers;

#[cfg(feature = "type_checker_derive")]
#[doc(hidden)]
pub use type_checker_derive::*;

pub trait Validator {
    fn check_validity(&self) -> Result<(), String>;
}
