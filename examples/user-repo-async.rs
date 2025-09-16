use ::async_trait::async_trait;

pub type User = &'static str;

#[async_trait]
pub trait UserRepository {
    async fn save(&mut self, user: User);
    async fn delete(&mut self, user: User);

    async fn contains(&self, user: User) -> bool;
}

pub struct UserRepositoryFeature;

impl UserRepositoryFeature {
    #[rustfmt::skip]
    #[allow(unused_braces)]
    #[allow(clippy::new_ret_no_self)]
    pub fn new<World>() -> impl ::litmus::IntoAsyncFeature<World>
    where
        World: UserRepository + ::core::marker::Send + ::core::marker::Sync + 'static,
    {
        ::litmus::AsyncFeature::new()
            .scenario(::litmus::AsyncScenario::<World>::new()
                .given("an empty repository", |_| ::litmus::r#async!())
                .when("inserting user `Alice`", |repo| ::litmus::r#async!({ repo.save("Alice").await; }))
                .then("it contains `Alice`", |repo| ::litmus::r#async!({ ::litmus::assert!(repo.contains("Alice").await) })))
            .scenario(::litmus::AsyncScenario::<World>::new()
                .given("a repository with user `Alice`", |repo| ::litmus::r#async!({ repo.save("Alice").await; }))
                .when("inserting user `Alice`", |repo| ::litmus::r#async!({ repo.save("Alice").await; }))
                .then("it contains `Alice`", |repo| ::litmus::r#async!({ ::litmus::assert!(repo.contains("Alice").await) })))
            .scenario(::litmus::AsyncScenario::<World>::new()
                .given("a repository with user `Alice`", |repo| ::litmus::r#async!({ repo.save("Alice").await; }))
                .when("deleting user `Alice`", |repo| ::litmus::r#async!({ repo.delete("Alice").await; }))
                .then("it does not contain `Alice`", |repo| ::litmus::r#async!({ ::litmus::assert!(!repo.contains("Alice").await) })))

            .scenario_outline(::litmus::AsyncScenarioOutline::new()
                .scenario(|user| ::litmus::AsyncScenario::<World>::new()
                    .given("an empty repository", |_| ::litmus::r#async!())
                    .when(::litmus::format!("inserting user {}", user), move |repo| ::litmus::r#async!({ repo.save(user).await; }))
                    .then(::litmus::format!("it contains {}", user), move |repo| ::litmus::r#async!({ ::litmus::assert!(repo.contains(user).await) })))
                .examples(["Alice", "Bob", "Charlie"]))
    }
}

#[derive(::core::default::Default)]
pub struct UserDatabase {
    users_by_ids: ::tokio::sync::Mutex<::std::collections::HashSet<User>>,
    is_connected: ::std::sync::atomic::AtomicBool,
}

impl UserDatabase {
    pub async fn connect(&mut self) {
        // self.is_connected.store(true, ::std::sync::atomic::Ordering::SeqCst);
    }

    pub async fn disconnect(&mut self) {
        // self.is_connected.store(false, ::std::sync::atomic::Ordering::SeqCst);
    }
}

#[async_trait]
impl UserRepository for UserDatabase {
    async fn save(&mut self, user: User) {
        if !self.is_connected.load(::std::sync::atomic::Ordering::SeqCst) {
            self.users_by_ids.lock().await.insert(user);
        }
    }

    async fn delete(&mut self, user: User) {
        if !self.is_connected.load(::std::sync::atomic::Ordering::SeqCst) {
            self.users_by_ids.lock().await.remove(user);
        }
    }

    async fn contains(&self, user: User) -> bool {
        if !self.is_connected.load(::std::sync::atomic::Ordering::SeqCst) {
            self.users_by_ids.lock().await.contains(&user)
        } else {
            false
        }
    }
}

// pub struct UserDatabaseSuite;

// impl UserDatabaseSuite {
//     #[rustfmt::skip]
//     #[allow(clippy::new_ret_no_self)]
//     pub fn new() -> impl ::litmus::IntoAsyncSuite<UserDatabase> {
//         ::litmus::AsyncSuite::new()
//             .feature(UserRepositoryFeature::new())
//             .before_scenario(|db: &mut UserDatabase| ::litmus::r#async!({ db.connect().await; }))
//             .after_scenario(|db: &mut UserDatabase| ::litmus::r#async!({ db.disconnect().await; }))
//     }
// }

#[rustfmt::skip]
#[::tokio::main]
async fn main() -> ::std::process::ExitCode {
    ::litmus::AsyncRunner::new()
        // .suite(UserDatabaseSuite::new())
        .feature(UserRepositoryFeature::new::<UserDatabase>())
        .run().await
}
