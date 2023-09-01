use async_trait::async_trait;
use todo_core::{models::todo_item::TodoItem, repositories::todo_repository_trait::TodoRespositoryTrait};
use uuid::Uuid;

use crate::entities::todo_entity::TodoEntity;

pub struct TodoRepository {

}

impl TodoRepository {
    pub fn new() -> TodoRepository {
        TodoRepository {}
    }
}

#[async_trait]
impl TodoRespositoryTrait<TodoItem> for TodoRepository {
    async fn get_all(&self) -> Vec<TodoItem> {
        let mut items = Vec::new();

        let todo_entity_1 = TodoEntity {
            id: Uuid::new_v4(),
            title: "First Todo".to_string(),
            description: "This is the first todo".to_string(),
            completed: false,
        };

        let todo_entity_2 = TodoEntity {
            id: Uuid::new_v4(),
            title: "Second Todo".to_string(),
            description: "This is the second todo".to_string(),
            completed: false,
        };

        let todo_entity_3 = TodoEntity {
            id: Uuid::new_v4(),
            title: "Third Todo".to_string(),
            description: "This is the third todo".to_string(),
            completed: false,
        };

        items.push(todo_entity_1.into());
        items.push(todo_entity_2.into());
        items.push(todo_entity_3.into());

        items
    }
}