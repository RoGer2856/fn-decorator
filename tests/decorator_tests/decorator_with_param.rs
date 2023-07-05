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
