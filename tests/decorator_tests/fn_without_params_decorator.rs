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
