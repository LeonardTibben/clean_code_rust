use uuid::Uuid;

pub struct TodoItem {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub completed: bool,
}