# type-rules

<div>
  <a href="https://crates.io/crates/type_rules">
    <img src="https://img.shields.io/crates/v/type_rules.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <a href="https://docs.rs/type_rules">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <a href="https://crates.io/crates/type_rules">
    <img src="https://img.shields.io/crates/l/type_rules.svg?style=flat-square"
    alt="License" />
  </a>
</div>

A tool to easily constrain a struct and recover errors.

## Table of Contents

1. [Install](#install)
2. [Basic checking](#basic-checking)
3. [Advanced checking](#advanced-checking)
4. [Make your own rule](#make-your-own-rule)
5. [Rules list](#rules-list)

## Install

```toml
# Cargo.toml
[dependencies]
type-rules = { version = "0.2.2", features = ["derive", "regex"] }
```

## Basic checking

You can declare a struct and impose some constraints on each field
and check the validity like this:

```rust
use chrono::prelude::*;
use type_rules::prelude::*;

#[derive(Validator)]
struct NewUser {
    #[rule(MaxLength(100), RegEx(r"^\S+@\S+\.\S+"))]
    email: String,
    #[rule(MinMaxLength(8, 50))]
    password: String,
    #[rule(Opt(MaxRange(Utc::now())))]
    birth_date: Option<DateTime<Utc>>
}

let new_user = NewUser {
    email: "examples@examples.com".to_string(),
    password: "OPw$5%hJ".to_string(),
    birth_date: None,
};
assert!(new_user.check_validity().is_ok());
let new_user = NewUser {
    email: "examples@examples.com".to_string(),
    password: "O".to_string(),
    birth_date: None,
};
assert!(new_user.check_validity().is_err()); //Value is too short
```

Also works with enums :

```rust
use type_rules::prelude::*;

#[derive(Validator)]
enum MyEnum {
    Option1(#[rule(MaxLength(200))] String),
    Option2 {
        #[rule(MinMaxRange(1, 10))]
        integer: u32
    },
    Option3,
}
```

## Advanced checking

To check recursively, you can use the `Validate` rule

```rust
use type_rules::prelude::*;

#[derive(Validator)]
struct EmailWrapper(#[rule(MaxLength(100), RegEx(r"^\S+@\S+\.\S+"))] String);

#[derive(Validator)]
struct User {
    #[rule(Validate())]
    email: EmailWrapper,
    #[rule(MinMaxLength(8, 50))]
    password: String,
}
```

You can use expressions directly in rule derive attribute.

For example, you can use const or function directly in the rule parameters:

```rust
use type_rules::prelude::*;
use chrono::prelude::*;

#[derive(Validator)]
struct BirthDate(#[rule(MaxRange(Utc::now()))] DateTime<Utc>);
```

```rust
use type_rules::prelude::*;

#[derive(Validator)]
struct Range {
    #[rule(MaxRange(self.max))]
    min: u32,
    #[rule(MinRange(self.min))]
    max: u32,
};
```

Or use expressions to express a rule directly.
Here is an example of using a rule with more complex values:

```rust
use std::env;
use type_rules::prelude::*;

fn generate_max_payload_rule() -> MaxLength {
    MaxLength(match env::var("MAX_PAYLOAD") {
        Ok(val) => val.parse().unwrap_or_else(|_| 10000),
        Err(_) => 10000,
    })
}

#[derive(Validator)]
struct Payload(#[rule(generate_max_payload_rule())] String);
```

In this case the `generate_max_payload_rule` function is executed at each check

## Make your own rule

If you need a specific rule, just make a tuple struct (or struct if you make the declaration outside the struct
definition)
that implements the `Rule` feature :

```rust
use type_rules::prelude::*;

struct IsEven();

impl Rule<i32> for IsEven {
    fn check(&self, value: &i32) -> Result<(), String> {
        if value % 2 == 0 {
            Ok(())
        } else {
            Err("Value is not even".into())
        }
    }
}

#[derive(Validator)]
struct MyInteger(#[rule(IsEven())] i32);
```

## Rules list

Here a list of the rules you can find in this crate.

Each rule has its own [documentation](https://docs.rs/type-rules/latest/type_rules/rules/index.html)
with examples.

Check the length of any type that implements `AsRef<str>` such
as `String` or `&str`:

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

- `Opt`: Apply another rule to inner value of an `Option` ex: `Opt(MinMaxRange(1, 4))`
- `And`: Rule to ensure that 2 other rules are `Ok` ex: `And(MaxLength(1000), RegEx(r"^\S+@\S+\.\S+"))`
- `Or`: Rule to apply an Or condition on two other rules. ex: `Or(MaxRange(-1), MinRange(1))`
- `Eval`: Rule to constrain any type to a predicate ex: `Eval(predicate, "Error message")`
- `Validate`: Recursive checking ex: `Validate()`
- `In`: Rule to constrain a type to be `in` a collection
  ex: `In(["apple", "banana", "orange", "pear"], "Value need to be a fruit")`
- `All`: Rule to constrain a collection to valid the specified rule
  ex: `All(MinLength(1), "You can't use empty string")`
- `RegEx`: check if a type that implement `AsRef<str>` (String, &str, ...) matches the regex.
  You need the `regex` feature to use it.
  ex: `RegEx(r"^\S+@\S+\.\S+")`
