trait IntoI32 {
    fn into_i32(self) -> i32;
}

impl IntoI32 for i32 {
    fn into_i32(self) -> i32 { self }
}

impl IntoI32 for () {
    fn into_i32(self) -> i32 { 0 }
}

trait Foo {
    fn foo(self) -> Box<dyn FnOnce(i32) -> i32>;
}

impl<F, R> Foo for F
where
    F: FnOnce(i32) -> R + 'static,
    R: IntoI32,
{
    fn foo(self) -> Box<dyn FnOnce(i32) -> i32> {
        Box::new(move |i| (self)(i).into_i32())
    }
}

fn main() {
    let f1 = |x| x + 1;             // returns i32
    let f2 = |x| println!("{}", x); // returns ()

    use crate::Foo;

    let b1 = f1.foo();
    let b2 = f2.foo();

    println!("{}", b1(10)); // 11
    println!("{}", b2(10)); // 0
}
