use chrono::prelude::*;
use type_rules::Validator;
use type_rules::rules::{
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
    #[rule(
        MaxLength(200),
        RegEx(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})"
        )
    )]
    String,
);

#[derive(Validator)]
struct SignUpForm {
    #[rule(Validate())]
    email: Email,
    #[rule(MinMaxLength(8, 50))]
    password: String,
}

#[derive(Validator)]
struct PastDate(#[rule(MaxRange(Utc::now()))] DateTime<Utc>);

#[derive(Validator)]
struct FloatWrapper(#[rule(MinMaxRange(0_f32, 100_f32))] f32);

#[derive(Validator)]
struct VecWrapper<T>(#[rule(MinMaxSize(1, 50))] Vec<T>);