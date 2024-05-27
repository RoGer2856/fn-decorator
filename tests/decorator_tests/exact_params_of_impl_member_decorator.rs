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
