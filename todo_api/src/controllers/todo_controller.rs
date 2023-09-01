use std::sync::Arc;

use todo_core::{models::todo_item::TodoItem, services::{todo_service_trait::TodoServiceTrait, todo_service::TodoService}};
use todo_data::repositories::todo_repository::TodoRepository;

use axum::{Router, routing::get, Extension, http::StatusCode, Json, response::IntoResponse};

use crate::requests::todo_request::TodoResponse;

// We'll use a type alias for the service because the full type is quite long.
type Service = Extension<Arc<dyn TodoServiceTrait<TodoItem>>>;

// GET /todos
async fn get_todos(service: Service) -> impl IntoResponse {
    let todos = service.get_all().await;

    // Convert TodoItem to TodoResponse.
    let todos: Vec<TodoResponse> = todos.into_iter().map(|todo| todo.into()).collect();

    (StatusCode::ACCEPTED, Json(todos))
}

// Function to create and return some routes.
pub async fn create_todo_routes() -> Router {
    // Create a service and wrap it in an `Arc`
    let service: Arc<dyn TodoServiceTrait<TodoItem>> = Arc::new(TodoService::new(
        Box::new(TodoRepository::new()),
    ));

    Router::new()
        .route("/todos", get(get_todos))
        .layer(Extension(service))
}