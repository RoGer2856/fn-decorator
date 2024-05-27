use std::future::Future;

use fn_decorator::use_decorator;

async fn decorator<FutureType: Future<Output = String>>(
    middle: String,
    f: impl FnOnce(String) -> FutureType,
    right: String,
) -> String {
    let right = middle + &right;
    f(right).await
}

#[use_decorator(decorator("_middle_".to_string()), exact_parameters = [right])]
async fn concat(left: String, right: String) -> String {
    left + &right
}

#[tokio::test]
async fn hiding_params_of_async_fn_decorator() {
    let result = concat("left".into(), "right".into()).await;
    assert_eq!(result, "left_middle_right");
}
