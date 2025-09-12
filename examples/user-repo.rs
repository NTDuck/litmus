pub type User = &'static str;

pub trait UserRepository {
    fn save(&mut self, user: User);
    fn delete(&mut self, user: User);

    fn contains(&self, user: User) -> bool;
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
                .when("inserting user `Alice`", |repo| repo.save("Alice"))
                .then("it contains `Alice`", |repo| ::litmus::assert!(repo.contains("Alice"))))
            .scenario(::litmus::Scenario::<World>::new()
                .given("a repository with user `Alice`", |repo| repo.save("Alice"))
                .when("inserting user `Alice`", |repo| repo.save("Alice"))
                .then("it contains `Alice`", |repo| ::litmus::assert!(repo.contains("Alice"))))
            .scenario(::litmus::Scenario::<World>::new()
                .given("a repository with user `Alice`", |repo| repo.save("Alice"))
                .when("deleting user `Alice`", |repo| repo.delete("Alice"))
                .then("it does not contain `Alice`", |repo| ::litmus::assert!(!repo.contains("Alice"))))

            .scenario_outline(::litmus::ScenarioOutline::new()
                .scenario(|user| ::litmus::Scenario::<World>::new()
                    .given("an empty repository", |_| {})
                    .when(::litmus::format!("inserting user {}", user), move |repo| repo.save(user))
                    .then(::litmus::format!("it contains {}", user), move |repo| ::litmus::assert!(repo.contains(user))))
                .examples(["Alice", "Bob", "Charlie"]))
    }
}

#[derive(::core::default::Default)]
pub struct UserDatabase {
    users_by_ids: ::std::collections::HashSet<User>,
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
    fn save(&mut self, user: User) {
        if self.is_connected {
            self.users_by_ids.insert(user);
        }
    }

    fn delete(&mut self, user: User) {
        if self.is_connected {
            self.users_by_ids.remove(user);
        }
    }

    fn contains(&self, user: User) -> bool {
        if self.is_connected {
            self.users_by_ids.contains(&user)
        } else {
            false
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
