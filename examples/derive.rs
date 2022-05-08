use chrono::prelude::*;
use type_rules::rules::{
    MaxLength, MaxRange, MinMaxLength, MinMaxRange, MinMaxSize, RegEx, Validate,
};
use type_rules::Validator;

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

#[allow(dead_code)]
#[derive(Validator)]
enum MyEnum {
    Option1(#[rule(MaxLength(200))] String),
    Option2 {
        #[rule(MinMaxRange(1, 10))]
        integer: u32,
    },
    Option3,
}

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

fn main() {}
