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
