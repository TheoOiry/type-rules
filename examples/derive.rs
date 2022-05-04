use chrono::prelude::*;
use type_checker::Validator;
use type_checker::checkers::{
    MaxLength,
    MinMaxLength,
    MinMaxSize,
    MinMaxRange,
    MaxRange,
    RegEx,
    Validate
};

#[derive(Validator)]
struct Email(
    #[check(
        MaxLength(200),
        RegEx(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})"
        )
    )]
    String,
);

#[derive(Validator)]
struct SignUpForm {
    #[check(Validate())]
    email: Email,
    #[check(MinMaxLength(8, 50))]
    password: String,
}

#[derive(Validator)]
struct PastDate(#[check(MaxRange(Utc::now()))] DateTime<Utc>);

#[derive(Validator)]
struct FloatWrapper(#[check(MinMaxRange(0_f32, 100_f32))] f32);

#[derive(Validator)]
struct VecWrapper<T>(#[check(MinMaxSize(1, 50))] Vec<T>);