use async_trait::async_trait;

#[async_trait]
pub trait TodoServiceTrait<T> : Send + Sync {
    async fn get_all(&self) -> Vec<T>;
}