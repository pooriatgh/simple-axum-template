use domain::hello_world::HelloWorld;
use infrastructure::db::hello_world_db::HelloWorldDb;

use crate::error::CoreError;

#[derive(Debug, Clone)]
pub struct HelloWorldService {
    db: HelloWorldDb,
}

impl HelloWorldService {
    pub fn new(db: HelloWorldDb) -> Self {
        Self { db }
    }

    pub async fn get_hello_world(&self, key: String) -> Result<HelloWorld, CoreError> {
        let hello_world = self.db.get(&key);
        match hello_world {
            Some(hello_world) => Ok(hello_world),
            None => Err(CoreError::NotFound(key.to_string())),
        }
    }
}
