use anyhow::Result;

/// Trait for a physical storage backend (e.g., a database or external store).
///
/// This trait enables a pluggable architecture where you can replace the in‑memory
/// storage with an enterprise-level storage system.
pub trait PhysicalStorage: Send + Sync {
    fn put(&self, key: &str, value: &[u8]) -> Box<dyn std::future::Future<Output = Result<(), anyhow::Error>> + Unpin + Send>;
    fn get(&self, key: &str) -> Box<dyn std::future::Future<Output = Result<Option<Vec<u8>>, anyhow::Error>> + Unpin + Send>;
    fn delete(&self, key: &str) -> Box<dyn std::future::Future<Output = Result<(), anyhow::Error>> + Unpin + Send>;
}
// pub trait PhysicalStorage: Send + Sync {
//     async fn put(&self, key: &str, value: &[u8]) -> Result<()>;
//     async fn get(&self, key: &str) -> Result<Option<Vec<u8>>>;
//     async fn delete(&self, key: &str) -> Result<()>;
// }
