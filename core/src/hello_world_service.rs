use std::sync::Arc;

use domain::configuration::Configuration;
use domain::hello_world::HelloWorld;
use infrastructure::db::hello_world_db::HelloWorldDb;

use crate::error::CoreError;

#[derive(Debug, Clone)]
pub struct HelloWorldService {
    db: Arc<HelloWorldDb>,
    configuration: Arc<Configuration>,
}

impl HelloWorldService {
    pub fn new(db: Arc<HelloWorldDb>, configuration: Arc<Configuration>) -> Self {
        Self { db, configuration }
    }

    pub async fn get_hello_world(&self, key: String) -> Result<HelloWorld, CoreError> {
        // let hello_world = self.db.get(&key);
        // match hello_world {
        //     Some(hello_world) => Ok(hello_world),
        //     None => Err(CoreError::NotFound(key.to_string())),
        // }
        Ok(HelloWorld {
            message: "Hello, World!".to_string(),
        })
    }
}
