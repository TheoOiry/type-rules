pub mod rules;

#[cfg(feature = "type-rules-derive")]
#[doc(hidden)]
pub use type_rules_derive::*;

pub use rules::Rule;

pub trait Validator {
    fn check_validity(&self) -> Result<(), String>;
}
