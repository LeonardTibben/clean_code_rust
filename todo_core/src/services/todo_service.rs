use async_trait::async_trait;

use crate::{repositories::{todo_repository_trait::TodoRespositoryTrait}, models::todo_item::TodoItem};

use super::todo_service_trait::TodoServiceTrait;

pub struct TodoService {
    todo_repository: Box<dyn TodoRespositoryTrait<TodoItem>>
}

impl TodoService {
    pub fn new(todo_repository: Box<dyn TodoRespositoryTrait<TodoItem>>) -> TodoService {
        TodoService {
            todo_repository,
        }
    }
}

#[async_trait]
impl TodoServiceTrait<TodoItem> for TodoService {
    async fn get_all(&self) -> Vec<TodoItem> {
        self.todo_repository.get_all().await
    }
}
