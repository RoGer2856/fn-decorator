# fn-decorator

The crate contains macros for implementing wrapper functions around member or static functions.

## Using the `use_decorator`

**Decorator function**
```rust
fn decorator(f: fn() -> i64) -> i64 {
    f() + 1
}
```

**Decorating a function**
```rust
#[use_decorator(decorator())]
fn get_1() -> i64 {
    1
}
```

When `get_1()` is called, then `decorator(get_1)` is executed instead. The decorator function can decide whether it calls the received function or not.

There is also a `use_impl_decorator` macro that works in `impl` blocks.

Both macros can have the same parameters:
* Decorator function call that should be executed. This can contain parameters. See examples for exact usage!
* `hide_parameters = [...]`: if the decorator function signature does not match the decorated, then this list can be used to hide some parameters from the decorator function. Be aware that `hide_parameters` and `exact_parameters` cannot be given the same time.
* `exact_parameters = [...]`: if the decorator function signature does not match the decorated, then this list can be used to specified the exact parameters to be passed to the decorator function. Be aware that `hide_parameters` and `exact_parameters` cannot be given the same time.
* `override_return_type = <type>`: if the decorator return type does not match the decorated, then this list can be used to change the return type of the decorated
* `debug`: when this parameter is given, then the code will generate a compile error with the generated source code. This is useful for debugging purposes.

## Fully working examples

### Decorating a function that has no parameters
```rust
use fn_decorator::use_decorator;

fn decorator(f: fn() -> i64) -> i64 {
    f() + 1
}

#[use_decorator(decorator())]
fn get_1() -> i64 {
    1
}

#[test]
fn fn_without_params_decorator() {
    let result = get_1();
    assert_eq!(result, 2);
}
```

### Decorating a function that has parameters
```rust
use fn_decorator::use_decorator;

fn decorator(f: fn(x: i64) -> i64, x: i64) -> i64 {
    f(x) * 2
}

#[use_decorator(decorator())]
fn double(x: i64) -> i64 {
    x * 2
}

#[test]
fn decorator_without_params() {
    let result = double(2);
    assert_eq!(result, 8);
}
```

### Using a parameterized decorator function
```rust
use fn_decorator::use_decorator;

fn decorator(
    middle: String,
    f: fn(String, String) -> String,
    left: String,
    right: String,
) -> String {
    let left = left + &middle;
    f(left, right)
}

#[use_decorator(decorator("_middle_".to_string()))]
fn concat(left: String, right: String) -> String {
    left + &right
}

#[test]
fn decorator_with_param() {
    let result = concat("left".into(), "right".into());
    assert_eq!(result, "left_middle_right");
}
```

### Debugging an fn decorator
Please be aware that the tests does not contain this code, because it produces a compile time error.

```rust
use fn_decorator::use_decorator;

fn decorator(f: fn() -> i64) -> i64 {
    f() + 1
}

#[use_decorator(decorator(), debug)]
fn get_1() -> i64 {
    1
}

#[test]
fn debug_fn_decorator() {
    let result = get_1();
    assert_eq!(result, 2);
}
```

### Decorating a member function
```rust
use fn_decorator::use_impl_decorator;

fn decorator(f: fn(&MyStruct, y: i64) -> i64, receiver: &MyStruct, y: i64) -> i64 {
    f(receiver, y) + 1
}

struct MyStruct {
    x: i64,
}

impl MyStruct {
    #[use_impl_decorator(decorator())]
    fn add(&self, y: i64) -> i64 {
        self.x + y
    }
}

#[test]
fn impl_member_decorator() {
    let obj = MyStruct { x: 1 };
    let result = obj.add(1);
    assert_eq!(result, 3);
}
```

### Decorating a static member function
```rust
use fn_decorator::use_impl_decorator;

fn decorator(f: fn(x: i64) -> i64, x: i64) -> i64 {
    f(x) + 1
}

struct MyStruct;

impl MyStruct {
    #[use_impl_decorator(decorator())]
    fn double(x: i64) -> i64 {
        x * 2
    }
}

#[test]
fn impl_static_member_decorator() {
    let result = MyStruct::double(2);
    assert_eq!(result, 5);
}
```

### Decorating an async function
```rust
use std::future::Future;

use fn_decorator::use_decorator;

async fn decorator<FutureType: Future<Output = i64>>(f: fn(x: i64) -> FutureType, x: i64) -> i64 {
    f(x).await * 2
}

#[use_decorator(decorator())]
async fn double(x: i64) -> i64 {
    x * 2
}

#[tokio::test]
async fn async_decorator() {
    let result = double(2).await;
    assert_eq!(result, 8);
}
```

### Specifying exact parameter list of an async decorator using with an async function
```rust
use std::future::Future;

use fn_decorator::use_decorator;

async fn decorator<FutureType: Future<Output = String>>(
    middle: String,
    f: impl FnOnce(String) -> FutureType,
    right: String,
) -> String {
    let right = middle + &right;
    f(right).await
}

#[use_decorator(decorator("_middle_".to_string()), exact_parameters = [right])]
async fn concat(left: String, right: String) -> String {
    left + &right
}

#[tokio::test]
async fn hiding_params_of_async_fn_decorator() {
    let result = concat("left".into(), "right".into()).await;
    assert_eq!(result, "left_middle_right");
}
```

### Specifying exact parameter list of an async decorator using with an async member function
```rust
use std::future::Future;

use fn_decorator::use_impl_decorator;

async fn decorator<'a, FutureType: Future<Output = &'a String>>(
    middle: String,
    f: impl FnOnce(&'a mut MyStruct) -> FutureType,
    receiver: &'a mut MyStruct,
) -> &'a String {
    receiver.left.push_str(&middle);
    f(receiver).await
}

struct MyStruct {
    left: String,
}

impl MyStruct {
    #[use_impl_decorator(decorator("_middle_".to_string()), exact_parameters = [self])]
    async fn concat(&mut self, right: String) -> &String {
        self.left.push_str(&right);
        &self.left
    }
}

#[tokio::test]
async fn hiding_params_of_async_impl_member_decorator() {
    let mut obj = MyStruct {
        left: "left".into(),
    };
    let result = obj.concat("right".into()).await;
    assert_eq!(result, "left_middle_right");
}
```

### Specifying exact parameter list of a decorator using with a function
```rust
use fn_decorator::use_decorator;

fn decorator(middle: String, f: impl FnOnce(String) -> String, right: String) -> String {
    let right = middle + &right;
    f(right)
}

#[use_decorator(decorator("_middle_".to_string()), exact_parameters = [right])]
fn concat(left: String, right: String) -> String {
    left + &right
}

#[test]
fn hiding_params_of_fn_decorator() {
    let result = concat("left".into(), "right".into());
    assert_eq!(result, "left_middle_right");
}
```

### Specifying exact parameter list of a decorator using with a member function
```rust
use fn_decorator::use_impl_decorator;

fn decorator(middle: String, f: impl FnOnce(&MyStruct) -> String, receiver: &MyStruct) -> String {
    let new_struct = MyStruct {
        left: receiver.left.clone() + &middle,
    };
    f(&new_struct)
}

struct MyStruct {
    left: String,
}

impl MyStruct {
    #[use_impl_decorator(decorator("_middle_".to_string()), exact_parameters = [self])]
    fn concat(&self, right: String) -> String {
        self.left.clone() + &right
    }
}

#[test]
fn hiding_params_of_impl_member_decorator() {
    let obj = MyStruct {
        left: "left".into(),
    };
    let result = obj.concat("right".into());
    assert_eq!(result, "left_middle_right");
}
```

#[test]
fn hiding_self_param_in_impl_member_decorator() {
    let obj = MyStruct {
        left: "left".into(),
    };
    let result = obj.concat("right".into());
    assert_eq!(result, "left_middle_right");
}
```

### Hiding parameters of an async function from a decorator
```rust
use std::future::Future;

use fn_decorator::use_decorator;

async fn decorator<FutureType: Future<Output = String>>(
    middle: String,
    f: impl FnOnce(String) -> FutureType,
    right: String,
) -> String {
    let right = middle + &right;
    f(right).await
}

#[use_decorator(decorator("_middle_".to_string()), hide_parameters = [left])]
async fn concat(left: String, right: String) -> String {
    left + &right
}

#[tokio::test]
async fn hiding_params_of_async_fn_decorator() {
    let result = concat("left".into(), "right".into()).await;
    assert_eq!(result, "left_middle_right");
}
```

### Hiding parameters of an async member function from a decorator
```rust
use std::future::Future;

use fn_decorator::use_impl_decorator;

async fn decorator<'a, FutureType: Future<Output = &'a String>>(
    middle: String,
    f: impl FnOnce(&'a mut MyStruct) -> FutureType,
    receiver: &'a mut MyStruct,
) -> &'a String {
    receiver.left.push_str(&middle);
    f(receiver).await
}

struct MyStruct {
    left: String,
}

impl MyStruct {
    #[use_impl_decorator(decorator("_middle_".to_string()), hide_parameters = [right])]
    async fn concat(&mut self, right: String) -> &String {
        self.left.push_str(&right);
        &self.left
    }
}

#[tokio::test]
async fn hiding_params_of_async_impl_member_decorator() {
    let mut obj = MyStruct {
        left: "left".into(),
    };
    let result = obj.concat("right".into()).await;
    assert_eq!(result, "left_middle_right");
}
```

### Hiding parameters of a function from a decorator
```rust
use fn_decorator::use_decorator;

fn decorator(middle: String, f: impl FnOnce(String) -> String, right: String) -> String {
    let right = middle + &right;
    f(right)
}

#[use_decorator(decorator("_middle_".to_string()), hide_parameters = [left])]
fn concat(left: String, right: String) -> String {
    left + &right
}

#[test]
fn hiding_params_of_fn_decorator() {
    let result = concat("left".into(), "right".into());
    assert_eq!(result, "left_middle_right");
}
```

### Hiding parameters of a member function from a decorator
```rust
use fn_decorator::use_impl_decorator;

fn decorator(middle: String, f: impl FnOnce(&MyStruct) -> String, receiver: &MyStruct) -> String {
    let new_struct = MyStruct {
        left: receiver.left.clone() + &middle,
    };
    f(&new_struct)
}

struct MyStruct {
    left: String,
}

impl MyStruct {
    #[use_impl_decorator(decorator("_middle_".to_string()), hide_parameters = [right])]
    fn concat(&self, right: String) -> String {
        self.left.clone() + &right
    }
}

#[test]
fn hiding_params_of_impl_member_decorator() {
    let obj = MyStruct {
        left: "left".into(),
    };
    let result = obj.concat("right".into());
    assert_eq!(result, "left_middle_right");
}
```

### Hiding self of a member function from a decorator
```rust
use fn_decorator::use_impl_decorator;

fn decorator(middle: String, f: impl FnOnce(String) -> String, right: String) -> String {
    let right = middle + &right;
    f(right)
}

struct MyStruct {
    left: String,
}

impl MyStruct {
    #[use_impl_decorator(decorator("_middle_".to_string()), hide_parameters = [self])]
    fn concat(&self, right: String) -> String {
        self.left.clone() + &right
    }
}

#[test]
fn hiding_self_param_in_impl_member_decorator() {
    let obj = MyStruct {
        left: "left".into(),
    };
    let result = obj.concat("right".into());
    assert_eq!(result, "left_middle_right");
}
```

### Decorating an async member function
```rust
use std::future::Future;

use fn_decorator::use_impl_decorator;

async fn decorator<'a, FutureType: Future<Output = i64>>(
    f: fn(&'a MyStruct, y: i64) -> FutureType,
    receiver: &'a MyStruct,
    y: i64,
) -> i64 {
    f(receiver, y).await + 1
}

struct MyStruct {
    x: i64,
}

impl MyStruct {
    #[use_impl_decorator(decorator())]
    async fn add(&self, y: i64) -> i64 {
        self.x + y
    }
}

#[tokio::test]
async fn async_impl_member_decorator() {
    let obj = MyStruct { x: 1 };
    let result = obj.add(1).await;
    assert_eq!(result, 3);
}
```

### Decorating an async static member function
```rust
use std::future::Future;

use fn_decorator::use_impl_decorator;

async fn decorator<FutureType: Future<Output = i64>>(f: fn(y: i64) -> FutureType, x: i64) -> i64 {
    f(x).await + 1
}

struct MyStruct;

impl MyStruct {
    #[use_impl_decorator(decorator())]
    async fn double(x: i64) -> i64 {
        x * 2
    }
}

#[tokio::test]
async fn async_impl_member_decorator() {
    let result = MyStruct::double(2).await;
    assert_eq!(result, 5);
}
```

### Overriding return type
```rust
use fn_decorator::use_decorator;

fn decorator(f: fn() -> i64) -> Result<i64, ()> {
    Ok(f() + 1)
}

#[use_decorator(decorator(), override_return_type = Result<i64, ()>)]
fn get_1() -> i64 {
    1
}

#[use_decorator(decorator(), override_return_type = impl ::std::any::Any)]
fn get_2() -> i64 {
    2
}

#[test]
fn overriding_return_type() {
    let result = get_1();
    assert_eq!(result, Ok(2));

    let result = get_2();
    assert_eq!("core::result::Result<i64, ()>", ::std::any::type_name_of_val(&result));
}
```
