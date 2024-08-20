use super::in_memory_db::InMemoryDb;
use dashmap::DashMap;
use domain::hello_world::HelloWorld;

#[derive(Debug, Clone)]
pub struct HelloWorldDb {
    db: InMemoryDb<String, HelloWorld>,
}

impl HelloWorldDb {
    pub fn new() -> Self {
        Self {
            db: InMemoryDb {
                store: DashMap::new(),
            },
        }
    }

    pub fn get(&self, key: &str) -> Option<HelloWorld> {
        self.db.store.get(key).map(|x| x.value().clone())
    }
}
