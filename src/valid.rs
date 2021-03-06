use crate::Validator;
use std::fmt;
use std::ops::Deref;

/// `Valid` is a wrapper for any type that implements [`Validator`]
/// it permit to ensure at compile time that the inner type as been
/// verified.
///
/// With the [`serde`] feature, Valid can be serialized and deserialized
/// with validity check.
/// ```
/// use type_rules::prelude::*;
///
/// #[derive(Validator)]
/// struct NewUser {
///     #[rule(MinMaxLength(3, 50))]
///     username: String,
///     #[rule(MinMaxLength(8, 100))]
///     password: String,
/// }
///
/// fn do_something(user: Valid<NewUser>) {
///     // No need to check if user is valid
/// }
///
/// let new_user = NewUser {
///     username: "example".to_string(),
///     password: "OPw$5%hJJ".to_string(),
/// };
/// do_something(Valid::new(new_user).unwrap());
/// ```
#[derive(Debug)]
pub struct Valid<T: Validator>(T);

impl<T: Validator> Valid<T> {
    #[allow(dead_code)]
    pub fn new(val: T) -> Result<Self, String> {
        val.check_validity()?;
        Ok(Valid(val))
    }

    #[allow(dead_code)]
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Validator> Deref for Valid<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Validator + fmt::Display> fmt::Display for Valid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'a, T: Validator + serde::Deserialize<'a>> serde::Deserialize<'a> for Valid<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let v = T::deserialize(deserializer)?;
        Valid::new(v).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<T: Validator + serde::Serialize> serde::Serialize for Valid<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use crate::valid::Valid;
    use crate::Validator;
    use claim::{assert_err, assert_ok};

    #[derive(Debug)]
    struct ValidTest(i32);

    impl Validator for ValidTest {
        fn check_validity(&self) -> Result<(), String> {
            match self.0.is_positive() {
                true => Ok(()),
                false => Err(String::from("Need to be positive")),
            }
        }
    }

    #[test]
    fn valid_ok() {
        assert_ok!(Valid::new(ValidTest(1)));
    }

    #[test]
    fn valid_err() {
        assert_err!(Valid::new(ValidTest(-1)));
    }

    #[cfg(feature = "serde")]
    mod serde_tests {
        use crate::valid::Valid;
        use crate::Validator;
        use claim::{assert_err, assert_ok};
        use serde_derive::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize, Debug)]
        struct Int {
            val: i32,
        }

        impl Validator for Int {
            fn check_validity(&self) -> Result<(), String> {
                match self.val.is_positive() {
                    true => Ok(()),
                    false => Err(String::from("Need to be positive")),
                }
            }
        }

        #[test]
        fn valid_serde_ok() {
            let serialized = "{\"val\":1}";
            let res: serde_json::Result<Valid<Int>> = serde_json::from_str(serialized);
            assert_ok!(res);
        }

        #[test]
        fn valid_serde_err() {
            let serialized = "{\"val\":-1}";
            let res: serde_json::Result<Valid<Int>> = serde_json::from_str(serialized);
            assert_err!(res);
        }

        #[test]
        fn valid_serde_serialize() {
            let valid_int = Valid::new(Int { val: 1 }).unwrap();
            let serialized = serde_json::to_string(&valid_int).unwrap();
            assert_eq!(&serialized, "{\"val\":1}")
        }
    }
}
