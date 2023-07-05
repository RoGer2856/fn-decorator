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
