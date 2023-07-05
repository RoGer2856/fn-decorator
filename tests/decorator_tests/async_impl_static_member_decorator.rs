use std::future::Future;

use fn_decorator::use_impl_decorator;

async fn decorator<FutureType: Future<Output = i64>>(f: fn(y: i64) -> FutureType, x: i64) -> i64 {
    f(x).await + 1
}

struct MyStruct;

impl MyStruct {
    #[use_impl_decorator(decorator())]
    async fn double(x: i64) -> i64 {
        x * 2
    }
}

#[tokio::test]
async fn async_impl_member_decorator() {
    let result = MyStruct::double(2).await;
    assert_eq!(result, 5);
}
