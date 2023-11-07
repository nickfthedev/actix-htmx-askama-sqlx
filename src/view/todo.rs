
use askama_actix::Template;

use crate::model::todo::TodoItem;
///
/// Template Structs
/// 

#[allow(dead_code)]
#[derive(Template)]
#[template(path = "todo_index.html")]
pub struct TodoIndex<> {}


#[allow(dead_code)]
#[derive(Template)]
#[template(path = "todo_list.html")]
pub struct TodoList<'a> {
    #[template(escape = "none")]
    pub todo: Vec<TodoItem<'a>>,
}

#[allow(dead_code)]
#[derive(Template)]
#[template(path = "todo_list_item_edit.html")]
pub struct TodoListItemEdit<'a> {
    pub id: &'a i32,
    pub task: &'a str,
    pub completed: &'a bool,
}