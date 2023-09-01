use async_trait::async_trait;

#[async_trait]
pub trait TodoRespositoryTrait<T>: Send + Sync {
    async fn get_all(&self) -> Vec<T>;
}