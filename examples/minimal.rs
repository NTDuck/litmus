use litmus::Background;

fn main() {
    let _i = Background::builder()
        .description("foo")
        .ignored(false)
        .given("something", || Ok(4))
        .and("then", |i| Ok(*i += 1))
        .but("some", |i| Ok(*i += 1))
        .build();
}
