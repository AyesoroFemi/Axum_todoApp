use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize)]
pub struct Todo {
    pub id: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TodoBody {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    todos_store: Arc<Mutex<Vec<Option<Todo>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            todos_store: Arc::new(Mutex::new(vec![
                Some(Todo {
                    id: 0,
                    title: String::from("Write to Rust code"),
                }),
                Some(Todo {
                    id: 1,
                    title: String::from("Write to TS code"),
                }),
            ])),
        })
    }
}

impl ModelController {
    pub async fn create_todo(&self, todo_body: TodoBody) -> Result<Todo> {
        let mut store = self.todos_store.lock().unwrap();

        let id = store.len() as u64;

        let todo = Todo {
            id,
            title: todo_body.title,
        };

        store.push(Some(todo.clone()));

        Ok(todo)
    }

    pub async fn list_todos(&self) -> Result<Vec<Todo>> {
        let store = self.todos_store.lock().unwrap();

        let todos = store.iter().filter_map(|todo| todo.clone()).collect();

        Ok(todos)
    }

    pub async fn delete_todo(&self, id: u64) -> Result<Todo> {
        let mut store = self.todos_store.lock().unwrap();

        let todo = store.get_mut(id as usize).and_then(|todo| todo.take());
        todo.ok_or(Error::InternalServerError)
    }
}
