use fn_decorator::use_impl_decorator;

fn decorator(f: fn(x: i64) -> i64, x: i64) -> i64 {
    f(x) + 1
}

struct MyStruct;

impl MyStruct {
    #[use_impl_decorator(decorator())]
    fn double(x: i64) -> i64 {
        x * 2
    }
}

#[test]
fn impl_static_member_decorator() {
    let result = MyStruct::double(2);
    assert_eq!(result, 5);
}
