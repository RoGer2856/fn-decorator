# fn-decorator

The crate contains macros for implementing wrapper functions around member or static functions.

## Decorator function examples

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
