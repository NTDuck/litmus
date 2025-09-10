pub type User = &'static str;
pub type UserId = u64;

pub trait UserRepository {
    fn save(&mut self, id: UserId, user: User);
    fn get(&self, id: &UserId) -> ::core::option::Option<&User>;
}

pub struct UserRepositoryFeature;

impl UserRepositoryFeature {
    #[rustfmt::skip]
    #[allow(clippy::new_ret_no_self)]
    pub fn new<World>() -> impl ::litmus::IntoFeature<World>
    where
        World: UserRepository + 'static,
    {
        ::litmus::Feature::new()
            .scenario(::litmus::Scenario::<World>::new()
                .given("an empty repository", |_| {})
                .when("inserting user `Alice` with ID `1`", |repo| repo.save(1, "Alice"))
                .then("it contains `Alice`", |repo| ::litmus::assert!(repo.get(&1) == Some(&"Alice"))))
            .scenario(::litmus::Scenario::<World>::new()
                .given("an empty repository", |_| {})
                .when("inserting user `Bob` with ID `2`", |repo| repo.save(2, "Bob"))
                .then("it contains `Bob`", |repo| ::litmus::assert!(repo.get(&2) == Some(&"Bob")))
                .but("it does not contain `Alice`", |repo| ::litmus::assert!(repo.get(&2) != Some(&"Alice"))))
            .scenario_outline(::litmus::ScenarioOutline::new()
                .scenario(|(id, user)| ::litmus::Scenario::<World>::new()
                    .given("an empty repository", |_| {})
                    .when(::litmus::format!("inserting user {user} with ID {id}"), move |repo| repo.save(id, user))
                    .then(::litmus::format!("it contains {user}"), move |repo| ::litmus::assert!(repo.get(&id) == Some(&user))))
                .examples([
                    (1, "Alice"),
                    (2, "Bob"),
                    (3, "Charlie"),
                ]))
    }
}

#[derive(::core::default::Default)]
pub struct UserDatabase {
    users_by_ids: ::std::collections::HashMap<UserId, User>,
    is_connected: bool,
}

impl UserDatabase {
    pub fn connect(&mut self) {
        self.is_connected = true;
    }

    pub fn disconnect(&mut self) {
        self.is_connected = false;
    }
}

impl UserRepository for UserDatabase {
    fn save(&mut self, id: UserId, user: User) {
        if self.is_connected {
            self.users_by_ids.insert(id, user);
        }
    }

    fn get(&self, id: &UserId) -> ::core::option::Option<&User> {
        if self.is_connected {
            self.users_by_ids.get(id)
        } else {
            None
        }
    }
}

pub struct UserDatabaseSuite;

impl UserDatabaseSuite {
    #[rustfmt::skip]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> impl ::litmus::IntoSuite<UserDatabase> {
        ::litmus::Suite::new()
            .feature(UserRepositoryFeature::new())
            .before_scenario(|db: &mut UserDatabase| db.connect())
            .after_scenario(|db: &mut UserDatabase| db.disconnect())
    }
}

#[rustfmt::skip]
fn main() -> ::std::process::ExitCode {
    ::litmus::Runner::new()
        .suite(UserDatabaseSuite::new())
        .run()
}
