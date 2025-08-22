use litmus::{Background, Scenario};

use std::{rc::Rc, sync::Arc};

fn main() {
    let _b = Background::builder()
        .description("background")
        .ignored(false)
        .given("given", || Ok(0))
        .and("given", |i| Ok(*i += 1))
        .but("given", |i| Ok(*i += 1))
        .build();

    // let _s = Scenario::builder()
    //     .description("scenario")
    //     .ignored(false)
    //     .tags(["tag0", "tag1"])
    //     .given("given", || Ok(0))
    //     .when("when", |i| Ok(*i += 1))
    //     .and("when", |i| Ok(*i += 1))
    //     .then("then", |i| Ok(assert!(*i == 1)))
    //     .build();
}
