use fn_decorator::use_impl_decorator;

fn decorator(f: fn(&MyStruct, y: i64) -> i64, receiver: &MyStruct, y: i64) -> i64 {
    f(receiver, y) + 1
}

struct MyStruct {
    x: i64,
}

impl MyStruct {
    #[use_impl_decorator(decorator())]
    fn add(&self, y: i64) -> i64 {
        self.x + y
    }
}

#[test]
fn impl_member_decorator() {
    let obj = MyStruct { x: 1 };
    let result = obj.add(1);
    assert_eq!(result, 3);
}
