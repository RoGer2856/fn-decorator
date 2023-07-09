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
