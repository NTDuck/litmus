# [litmus](https://en.wikipedia.org/wiki/Litmus)

a macro-free BDD test harness.<br>
inspired by [cucumber](https://crates.io/crates/cucumber) and [rspec](https://crates.io/crates/rspec).

## Why `litmus`
With `litmus`, you can ...<br><br>
write tests declaratively<br>
&emsp; with minimal overhead<br>
&emsp;&emsp; all without using macros.

## Major design criteria
- works with [cargo-test](https://doc.rust-lang.org/cargo/commands/cargo-test.html) and [cargo-nextest](https://nexte.st)
- offers [Gherkin](https://cucumber.io/docs/gherkin/)-ish ergonomics
- is [fast](https://blog.codinghorror.com/performance-is-a-feature/)

## Quickstart
Add this to your `Cargo.toml`:
```toml
# ./Cargo.toml

[dev-dependencies]
litmus = "0.5.0"
```

Disable the default harness for your test targets:
```toml
# ./Cargo.toml

[[test]]
name = ...
path = ...
harness = false
```

## Examples
The [following example](./examples/hello-world.rs) replicates the [cucumber-rs example](https://cucumber-rs.github.io/cucumber/main/).
```rust
/// ./examples/hello-world.rs

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
```

```toml
# ./Cargo.toml

[[example]]
name = "hello-world"
path = "./examples/hello-world.rs"
harness = false
```

We recommend [cargo-nextest](https://nexte.st) for a better experience, although [cargo-test](https://doc.rust-lang.org/cargo/commands/cargo-test.html) is supported.
```bash
$ cargo nextest run --example hello-world
   Compiling litmus v0.5.0 (D:\root\dev\rs\litmus)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.80s
────────────
 Nextest run ID c544a979-c5f2-4302-8f59-a387d06e66bc with nextest profile: default
    Starting 1 test across 1 binary
     Running [ 00:00:00] 0/1: 0 running, 0 passed, 0 skipped
        PASS [   0.025s] litmus::example/hello-world Eating a few isn't a problem
────────────
     Summary [   0.027s] 1 test run: 1 passed, 0 skipped
```
```bash
$ cargo test run --example hello-world
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running unittests examples\hello-world.rs (target\debug\examples\hello_world-149ad56ef15f2701.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.01s
```

More examples are available in the [`examples/`](./examples/) directory.

## License
This project is licensed under [the BSD 3-Clause License](./LICENSE).
