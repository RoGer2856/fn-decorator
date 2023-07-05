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
