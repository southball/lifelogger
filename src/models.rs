use thiserror::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Activity {
    pub id: String,
    pub title: String,
    pub description: String,

    pub todo_id: Option<String>,
}

/// This is the shared state that is shared across client and server.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SharedState {
    pub todos: im::HashMap<String, Todo>,
}

pub enum TodoMutation {
    Add(Todo),
    Update(Todo),
}

pub enum Mutation {
    Todo(TodoMutation),
}

#[derive(Error, Debug)]
pub enum UpdateError {
    #[error("Duplicate ID {0} when adding todo")]
    DuplicateAddTodoID(String),
    #[error("ID {0} not found when updating todo")]
    NotFoundUpdateTodoID(String),
}

impl SharedState {
    pub fn update(&self, mutation: Mutation) -> Result<SharedState, UpdateError> {
        Ok(match mutation {
            Mutation::Todo(todo_mutation) => match todo_mutation {
                TodoMutation::Add(todo) => SharedState {
                    todos: {
                        let mut todos = self.todos.clone();
                        if todos.contains_key(&todo.id) {
                            return Err(UpdateError::DuplicateAddTodoID(todo.id));
                        }
                        let id = todo.id.clone();
                        todos.insert(id, todo);
                        todos
                    },
                    ..(self.clone())
                },
                TodoMutation::Update(todo) => SharedState {
                    todos: {
                        let mut todos = self.todos.clone();
                        if !todos.contains_key(&todo.id) {
                            return Err(UpdateError::NotFoundUpdateTodoID(todo.id));
                        }
                        let id = todo.id.clone();
                        todos.insert(id, todo);
                        todos
                    },
                    ..(self.clone())
                },
            },
        })
    }
}
