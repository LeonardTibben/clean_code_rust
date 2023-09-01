use todo_core::models::todo_item::TodoItem;
use uuid::Uuid;

pub struct TodoEntity {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl From<TodoItem> for TodoEntity {
    fn from(item: TodoItem) -> Self {
        TodoEntity {
            id: item.id,
            title: item.title,
            description: item.description,
            completed: item.completed,
        }
    }
}

impl From<TodoEntity> for TodoItem {
    fn from(item: TodoEntity) -> Self {
        TodoItem {
            id: item.id,
            title: item.title,
            description: item.description,
            completed: item.completed,
        }
    }
}
