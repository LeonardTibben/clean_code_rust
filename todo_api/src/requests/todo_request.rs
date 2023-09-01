use serde::{Serialize, Deserialize};
use todo_core::models::todo_item::TodoItem;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl From<TodoItem> for TodoResponse {
    fn from(item: TodoItem) -> Self {
        TodoResponse {
            id: item.id,
            title: item.title,
            description: item.description,
            completed: item.completed,
        }
    }
}



