use ::cucumber::World as _;

#[derive(::cucumber::World, ::core::fmt::Debug, ::core::default::Default)]
struct World {
    user: Option<String>,
    capacity: usize,
}

#[::cucumber::given(expr = "{word} is hungry")] // Cucumber Expression
async fn someone_is_hungry(w: &mut World, user: String) {
    w.user = Some(user);
}

#[::cucumber::when(regex = r"^(?:he|she|they) eats? (\d+) cucumbers?$")]
async fn eat_cucumbers(w: &mut World, count: usize) {
    w.capacity += count;
    
    assert!(w.capacity < 4, "{} exploded!", w.user.as_ref().unwrap());
}

#[::cucumber::then("she is full")]
async fn is_full(w: &mut World) {
    assert_eq!(w.capacity, 3, "{} isn't full!", w.user.as_ref().unwrap());
}

#[::tokio::main]
async fn main() {
    World::run("./examples/cucumber-rs/features/hello-world.feature").await;
}
