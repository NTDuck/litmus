use litmus::{Background, Scenario};

fn main() {
    let _b = Background::builder()
        .description("background")
        .ignored(false)
        .given("given", |i| *i = 0)
        .and("given", |i| *i += 1)
        .but("given", |i| *i += 1)
        .build();

    let _s = Scenario::builder()
        // .description("scenario")
        .ignored(false)
        .tags(["tag0", "tag1"])
        .given("0", |i| *i = 0)
        .when("adding 1", |i| *i += 1)
        .and("adding 1 again", |i| *i += 2)
        .then("it equals 2", |i| {
            if *i == 3 {
                Ok(())
            } else {
                Err("it does not equal 2".into())
            }
        })
        .and("it does not equal 49", |i| {
            if *i == 49 {
                Err("it equals 49".into())
            } else {
                Ok(())
            }
        })
        .build();

    // litmus::run([_s]);
}
