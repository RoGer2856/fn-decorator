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
