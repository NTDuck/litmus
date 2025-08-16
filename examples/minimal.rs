use litmus::Background;

fn main() {
    let i = Background::builder()
        .description("foo")
        .ignored(false)
        .given("something", || 4)
        .and("then", |i| *i += 1)
        .build();
}
