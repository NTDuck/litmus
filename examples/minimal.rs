fn main() -> ::std::process::ExitCode {
    let runner = ::litmus::Runner::builder()
        .include_ignored()
        .color(::litmus::config::Color::Auto)
        .format(::litmus::config::Format::Terse)
        .feature(::litmus::Feature::builder()
            .scenario(::litmus::Scenario::builder()
                .given("0", |i| *i = 0)
                .when("adding 1", |i| *i += 1)
                .and("adding 1 again", |i| *i += 1)
                .then("it equals 2", |i| ::litmus::assert!(*i == 2))
                .and("it does not equal 49", |i| ::litmus::assert!(*i != 49))
                .build())
            .build())
        .build();

    runner.run()

    /*
    ::litmus::Runner::new()
        .include_ignored()
        .color(::litmus::config::Color::Auto)
        .format(::litmus::config::Format::Terse)
        .feature(::litmus::Feature::new()
            .scenario(::litmus::Scenario::new()
                .given("0", |i| *i = 0)
                .when("adding 1", |i| *i += 1)
                .and("adding 1 again", |i| *i += 1)
                .then("it equals 2", |i| ::litmus::assert!(*i == 2))
                .and("it does not equal 49", |i| ::litmus::assert!(*i != 49))))
        .run();
     */
}
