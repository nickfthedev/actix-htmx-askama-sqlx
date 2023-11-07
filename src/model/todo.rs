
use serde::Deserialize;

//
/// Structs
///

pub struct TodoItem<'a> {
    pub id: &'a i32,
    pub task: &'a str,
    pub completed: bool,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct AddTodo {
    pub task: String,
}


#[allow(dead_code)]
#[derive(Deserialize)]
pub struct UpdateTodo {
    pub id: i32,
    pub task: String,
    pub completed: Option<bool>, 
}

impl UpdateTodo {
    // A function that converts `completed` to a `bool` with a default value of `false`.
    pub fn get_completed(&self) -> bool {
        self.completed.unwrap_or(false)
    }
}