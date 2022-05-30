use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Builder)]
pub struct Todo {
    #[builder(default = "uuid::Uuid::new_v4().to_string()")]
    pub id: String,
    pub title: String,
    pub description: String,
    #[builder(default = "false")]
    pub completed: bool,

    #[builder(default = "chrono::Utc::now()")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, PartialEq, Builder)]
pub struct Activity {
    #[builder(default = "uuid::Uuid::new_v4().to_string()")]
    pub id: String,
    pub title: String,
    pub description: String,

    #[builder(default)]
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

impl From<TodoMutation> for Mutation {
    fn from(todo_mutation: TodoMutation) -> Mutation {
        Mutation::Todo(todo_mutation)
    }
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
