use ::async_trait::async_trait;

#[async_trait]
trait Repository {
    type Id: ::core::marker::Send + ::core::marker::Sync;
    type Entity: ::core::marker::Send + ::core::marker::Sync;

    async fn save(self: ::std::sync::Arc<Self>, id: Self::Id, entity: Self::Entity);
    async fn get<IdRef>(self: ::std::sync::Arc<Self>, id: IdRef) -> ::core::option::Option<Self::Entity>
    where
        IdRef: AsRef<Self::Id> + ::core::marker::Send + ::core::marker::Sync;
}

#[derive(::core::default::Default)]
struct Database<Id, Entity> {
    buffer: ::tokio::sync::Mutex<::std::collections::HashMap<Id, Entity>>,
    is_connected: ::std::sync::atomic::AtomicBool,
}

impl<Id, Entity> Database<Id, Entity> {
    async fn connect(self: ::std::sync::Arc<Self>) {
        self.is_connected.store(true, ::std::sync::atomic::Ordering::Relaxed);
    }

    async fn disconnect(self: ::std::sync::Arc<Self>) {
        self.is_connected.store(false, ::std::sync::atomic::Ordering::Relaxed);
    }
}

#[async_trait]
impl<Id, Entity> Repository for Database<Id, Entity>
where
    Id: ::std::cmp::Eq + ::std::hash::Hash + ::core::marker::Send + ::core::marker::Sync,
    Entity: ::core::clone::Clone + ::core::marker::Send + ::core::marker::Sync,
{
    type Id = Id;
    type Entity = Entity;

    async fn save(self: ::std::sync::Arc<Self>, id: Self::Id, entity: Self::Entity) {
        if self.is_connected.load(::std::sync::atomic::Ordering::Relaxed) {
            self.buffer.lock().await.insert(id, entity);
        }
    }

    async fn get<IdRef>(self: ::std::sync::Arc<Self>, id: IdRef) -> ::core::option::Option<Self::Entity>
    where
        IdRef: AsRef<Self::Id> + ::core::marker::Send + ::core::marker::Sync,
    {
        if self.is_connected.load(::std::sync::atomic::Ordering::Relaxed) {
            self.buffer.lock().await.get(id.as_ref()).cloned()
        } else {
            None
        }
    }
}

fn main() {

}