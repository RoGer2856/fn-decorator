use std::future::Future;

use fn_decorator::use_decorator;

async fn decorator<FutureType: Future<Output = i64>>(f: fn(x: i64) -> FutureType, x: i64) -> i64 {
    f(x).await * 2
}

#[use_decorator(decorator())]
async fn double(x: i64) -> i64 {
    x * 2
}

#[tokio::test]
async fn async_decorator() {
    let result = double(2).await;
    assert_eq!(result, 8);
}
