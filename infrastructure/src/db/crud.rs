use super::{error::DatabaseError, in_memory_db::InMemoryDb};

pub trait Crud<K, V> {
    /// Create a new value based on key and value.
    fn create(&self, id: K, value: V) -> Result<V, DatabaseError>;
    /// Read the value based on key.
    fn read(&self, id: &K) -> Result<Option<V>, DatabaseError>;
    /// Read all values.
    fn read_all(&self) -> Result<Vec<V>, DatabaseError>;
    /// Update the value based on key and return it.
    fn update(&self, id: K, value: V) -> Result<V, DatabaseError>;
    /// Delete the value based on key and return it.
    fn delete(&self, id: &K) -> Result<Option<V>, DatabaseError>;
    /// Check if the value exists based on key.
    fn exists(&self, id: &K) -> Result<bool, DatabaseError> {
        Ok(self.read(id)?.is_some())
    }
}

impl<K, V> Crud<K, V> for InMemoryDb<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
    V: Clone,
{
    fn create(&self, id: K, value: V) -> Result<V, DatabaseError> {
        self.store.insert(id, value.clone());
        Ok(value)
    }

    fn read(&self, id: &K) -> Result<Option<V>, DatabaseError> {
        Ok(self.store.get(id).map(|x| x.value().clone()))
    }

    fn read_all(&self) -> Result<Vec<V>, DatabaseError> {
        Ok(self.store.iter().map(|x| x.value().clone()).collect())
    }

    fn update(&self, id: K, value: V) -> Result<V, DatabaseError> {
        self.store.insert(id, value.clone());
        Ok(value)
    }

    fn delete(&self, id: &K) -> Result<Option<V>, DatabaseError> {
        Ok(self.store.remove(id).map(|x| x.1))
    }

    fn exists(&self, id: &K) -> Result<bool, DatabaseError> {
        Ok(self.store.contains_key(id))
    }
}
