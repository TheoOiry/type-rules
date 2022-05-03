# type-checker

A tool to easly constraint a struct and retrieves errors 

## Table of Contents

1. [Install](#install)
2. [Basic checking](#basic-checking)
3. [Advanced checking](#advanced-checking)
4. [Make your own checker](#make-your-own-checker)
5. [Checkers list](#checkers-list)

## Install
```toml
# Cargo.toml
[dependencies]
type_checker = { version = "0.1", features = ["derive"] }
```

## Basic checking

You can declare a struct and impose some constraints on each field:

```rust
use type_checker::Validator;
//Don't forget to import the checkers used.
use type_checker::checkers::{MaxLength, MinMaxLength, RegEx};

#[derive(Validator)]
struct NewUser {
    #[check(MaxLength(100), RegEx(r"^\S+@\S+\.\S+"))]
    email: String,
    #[check(MinMaxLength(8, 50))]
    password: String,
}
```

Then check the validity like this:

```rust
let new_user = NewUser {
    email: "example@example.com".to_string(),
    password: "OPw$5%hJ".to_string(),
};
new_user.check_validity().unwrap(); //OK
let new_user = NewUser {
    email: "example@example.com".to_string(),
    password: "O".to_string(),
};
new_user.check_validity().unwrap(); //Value is too short
```

## Advanced checking

To check recursively, you can use the `Validate` checker

```rust
#[derive(Validator)]
struct EmailWrapper(#[check(MaxLength(100), RegEx(r"^\S+@\S+\.\S+"))] String);

#[derive(Validator)]
struct User {
    #[check(Validate())]
    email: EmailWrapper,
    #[check(MinMaxLength(8, 50))]
    password: String,
}
```

You can use expressions directly in check derive attribute.

For example you can use const or function directly in the checker parameters:

```rust
use chrono::prelude::*;
use type_checker::Validator;
use type_checker::checkers::{MaxRange, MinLength};

const MIN_PASSWORD_LENGTH: usize = 8;

#[derive(Validator)]
struct Password(#[check(MinLength(MIN_PASSWORD_LENGTH))] String);

#[derive(Validator)]
struct PastDate(#[check(MaxRange(Utc::now()))] DateTime<Utc>);
```

Or use expressions to express a checker directly.
Here is an example of using a checker with more complex values:

```rust
fn generate_max_payload_checker() -> MaxLength {
    MaxLength(match env::var("MAX_PAYLOAD") {
        Ok(val) => val.parse::<usize>().unwrap_or_else(|_| 10000),
        Err(_) => 10000,
    })
}

#[derive(Validator)]
struct Payload(#[check(generate_max_payload_checker())] String);
```

In this case the `generate_max_payload_checker` function is executed at each check

## Make your own checker

If you need a specific checker, just make a tuple struct (or struct if you make the declaration outside the struct definition)
that implements the `Checker` feature :

```rust
use type_checker::checkers::Checker;
use type_checker::Validator;

struct IsEven();

impl Checker<i32> for IsEven {
    fn check(&self, value: &i32) -> Result<(), String> {
        if value % 2 == 0 {
            Ok(())
        } else {
            Err("Value is not even".into())
        }
    }
}

#[derive(Validator)]
struct MyInteger(#[check(IsEven())] i32);
```

## Checkers list

Check the length of a `String` or `&str`:
- `MinLength`: Minimum length ex: `MinLength(5)`
- `MaxLength`: Maximum length ex: `MaxLength(20)`
- `MinMaxLength`: Minimum and maximum length ex: `MinMaxLength(5, 20)`

Check the range for anything that implements `PartialOrd<Self>` like all numeric/floating types
or dates with `chrono`:
- `MinRange`: Minimum range ex: `MinRange(5)`
- `MaxRange`: Maximum range ex: `MaxRange(20)`
- `MinMaxRange`: Minimum and maximum range ex: `MinMaxRange(5, 20)`

Check the size of a `Vec<T>` :
- `MinSize`: Minimum size ex: `MinSize(5)`
- `MaxSize`: Maximum size ex: `MaxSize(20)`
- `MinMaxSize`: Minimum and maximum size ex: `MinMaxSize(5, 20)`

others :

- `Validate`: Recursive checking ex: `Validate()`
- `RegEx`: check if a `String` or `&str` matches the regex ex: `RegEx(r"^\S+@\S+\.\S+")`
