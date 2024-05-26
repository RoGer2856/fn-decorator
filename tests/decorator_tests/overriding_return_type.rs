use fn_decorator::use_decorator;

fn decorator(f: fn() -> i64) -> Result<i64, ()> {
    Ok(f() + 1)
}

#[use_decorator(decorator(), override_return_type = Result<i64, ()>)]
fn get_1() -> i64 {
    1
}

#[test]
fn fn_without_params_decorator() {
    let result = get_1();
    assert_eq!(result, Ok(2));
}
