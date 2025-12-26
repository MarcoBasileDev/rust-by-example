use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTodoFormModel {
    pub task: String,
}

#[derive(Deserialize)]
pub struct MarkTodoAsDoneFormModel {
    pub is_done: bool,
}
