use crate::Validator;
use std::ops::Deref;

#[derive(Debug, Copy, Clone)]
pub struct Valid<T>(T);

impl<T: Validator> Valid<T> {
    #[allow(dead_code)]
    pub fn new(val: T) -> Result<Self, String> {
        val.check_validity()?;
        Ok(Valid(val))
    }
}

impl<T> Deref for Valid<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
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
impl<T: serde::Serialize> serde::Serialize for Valid<T> {
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
