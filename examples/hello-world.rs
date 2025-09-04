#[derive(::core::default::Default)]
struct World {
    user: ::core::option::Option<String>,
    capacity: usize,
}

#[rustfmt::skip]
fn main() -> ::std::process::ExitCode {
    ::litmus::Runner::new()
        .include_ignored()
        .color(::litmus::config::Color::Auto)
        .format(::litmus::config::Format::Terse)
        .feature(::litmus::Feature::new()
            .description("Eating too much cucumbers may not be good for you")
            .scenario(::litmus::Scenario::<World>::new()
                .description("Eating a few isn't a problem")
                .given("Alice is hungry", |w| w.user = Some("Alice".to_owned()))
                .when("she eats 3 cucumbers", |w| {
                    w.capacity += 3;
                    ::litmus::assert!(w.capacity < 4, "Alice exploded")
                })
                .then("she is full", |w| ::litmus::assert!(w.capacity == 3, "Alice isn't full!"))))
        .run()
}
