use private::get_1;

fn decorator(f: fn() -> i64) -> i64 {
    f() + 1
}

mod private {
    use fn_decorator::use_decorator;

    use super::decorator;

    #[use_decorator(decorator())]
    pub fn get_1() -> i64 {
        1
    }
}

#[test]
fn fn_without_params_decorator() {
    let result = get_1();
    assert_eq!(result, 2);
}
