use dashmap::DashMap;

#[derive(Debug, Clone)]
pub struct InMemoryDb<K: std::cmp::Eq + std::hash::Hash, V> {
    pub store: DashMap<K, V>,
}
