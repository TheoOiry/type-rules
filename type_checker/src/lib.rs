
pub mod checkers;

pub use type_checker_derive::*;

pub trait Validator {
    fn check_validity(&self) -> Result<(), String>;
}
