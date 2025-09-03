#[derive(::core::default::Default)]
struct World {
    user: Option<String>,
    capacity: usize,
}

fn main() -> ::std::process::ExitCode {
    let runner = ::litmus::Runner::builder()
        .include_ignored()
        .color(::litmus::config::Color::Auto)
        .format(::litmus::config::Format::Terse)
        .feature(::litmus::Feature::builder()
            .description("Eating too much cucumbers may not be good for you")
            .scenario(::litmus::Scenario::builder()
                .given("Alice is hungry", |w: &mut World| w.user = Some("Alice".to_owned()))
                .when("she eats 3 cucumbers", |w| {
                    w.capacity += 3;
                    ::litmus::assert!(w.capacity < 4, "Alice exploded")
                })
                .then("she is full", |w| {
                    ::litmus::assert!(w.capacity == 3, "Alice isn't full!")
                })
                .build())
            .build())
        .build();

    runner.run()
}
