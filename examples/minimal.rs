use litmus::{Background, Scenario};

fn main() {
    let _b = Background::builder()
        .description("background")
        .ignored(false)
        .given("given", || Ok(0))
        .and("given", |i| Ok(*i += 1))
        .but("given", |i| Ok(*i += 1))
        .build();

    let _s = Scenario::builder()
        .description("scenario")
        .ignored(false)
        .tags(["tag0", "tag1"])
        .given("0", || Ok(0))
        .when("adding 1", |i| Ok(*i += 1))
        .and("adding 1 again", |i| Ok(*i += 1))
        .then("it equals 1", |i| Ok(assert!(*i == 1)))
        .and("it does not equal 2", |i| Ok(assert!(*i != 2)))
        .build();

    litmus::run([_s]);
}
