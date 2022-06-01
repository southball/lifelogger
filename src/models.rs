use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Builder, Serialize, Deserialize)]
pub struct Topic {
    #[builder(default = "uuid::Uuid::new_v4().to_string()")]
    pub id: String,
    pub name: String,
    #[builder(default)]
    pub parent_id: Option<String>,
    #[builder(default = "false")]
    pub archived: bool,
}

#[derive(Clone, Debug, PartialEq, Builder, Serialize, Deserialize)]
pub struct Todo {
    #[builder(default = "uuid::Uuid::new_v4().to_string()")]
    pub id: String,
    pub title: String,
    pub description: String,
    #[builder(default = "false")]
    pub completed: bool,
    #[builder(default = "chrono::Utc::now()")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[builder(default)]
    pub deadline: Option<chrono::DateTime<chrono::Utc>>,
    #[builder(default)]
    pub topic_id: Option<String>,
    #[builder(default = "false")]
    pub archived: bool,
}

#[derive(Clone, Debug, PartialEq, Builder, Serialize, Deserialize)]
pub struct Activity {
    #[builder(default = "uuid::Uuid::new_v4().to_string()")]
    pub id: String,
    pub title: String,
    pub description: String,
    #[builder(default)]
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    #[builder(default)]
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    #[builder(default)]
    pub todo_id: Option<String>,
    #[builder(default)]
    pub topic_id: Option<String>,
    #[builder(default = "false")]
    pub archived: bool,
}

/// This is the shared state that is shared across client and server.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SharedState {
    pub todos: im::HashMap<String, Todo>,
    pub activities: im::HashMap<String, Activity>,
    pub topics: im::HashMap<String, Topic>,
}

pub enum TodoMutation {
    Add(Todo),
    Update(Todo),
}

pub enum ActivityMutation {
    Add(Activity),
    Update(Activity),
}

pub enum TopicMutation {
    Add(Topic),
    Update(Topic),
}

pub enum Mutation {
    Todo(TodoMutation),
    Activity(ActivityMutation),
    Topic(TopicMutation),
}

impl From<TodoMutation> for Mutation {
    fn from(todo_mutation: TodoMutation) -> Mutation {
        Mutation::Todo(todo_mutation)
    }
}

impl From<ActivityMutation> for Mutation {
    fn from(activity_mutation: ActivityMutation) -> Mutation {
        Mutation::Activity(activity_mutation)
    }
}

impl From<TopicMutation> for Mutation {
    fn from(topic_mutation: TopicMutation) -> Mutation {
        Mutation::Topic(topic_mutation)
    }
}

#[derive(Error, Debug)]
pub enum UpdateError {
    #[error("Duplicate ID {0} when adding todo")]
    DuplicateAddTodoID(String),
    #[error("ID {0} not found when updating todo")]
    NotFoundUpdateTodoID(String),
    #[error("Duplicate ID {0} when adding activity")]
    DuplicateAddActivityID(String),
    #[error("ID {0} not found when updating activity")]
    NotFoundUpdateActivityID(String),
    #[error("Duplicate ID {0} when adding topic")]
    DuplicateAddTopicID(String),
    #[error("ID {0} not found when updating topic")]
    NotFoundUpdateTopicID(String),
}

impl SharedState {
    pub fn update(&self, mutation: Mutation) -> Result<SharedState, UpdateError> {
        Ok(match mutation {
            Mutation::Todo(TodoMutation::Add(todo)) => SharedState {
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
            Mutation::Todo(TodoMutation::Update(todo)) => SharedState {
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
            Mutation::Activity(ActivityMutation::Add(activity)) => SharedState {
                activities: {
                    let mut activities = self.activities.clone();
                    if activities.contains_key(&activity.id) {
                        return Err(UpdateError::DuplicateAddActivityID(activity.id));
                    }
                    let id = activity.id.clone();
                    activities.insert(id, activity);
                    activities
                },
                ..(self.clone())
            },
            Mutation::Activity(ActivityMutation::Update(activity)) => SharedState {
                activities: {
                    let mut activities = self.activities.clone();
                    if !activities.contains_key(&activity.id) {
                        return Err(UpdateError::NotFoundUpdateActivityID(activity.id));
                    }
                    let id = activity.id.clone();
                    activities.insert(id, activity);
                    activities
                },
                ..(self.clone())
            },
            Mutation::Topic(TopicMutation::Add(topic)) => SharedState {
                topics: {
                    let mut topics = self.topics.clone();
                    if topics.contains_key(&topic.id) {
                        return Err(UpdateError::DuplicateAddTopicID(topic.id));
                    }
                    let id = topic.id.clone();
                    topics.insert(id, topic);
                    topics
                },
                ..(self.clone())
            },
            Mutation::Topic(TopicMutation::Update(topic)) => SharedState {
                topics: {
                    let mut topics = self.topics.clone();
                    if !topics.contains_key(&topic.id) {
                        return Err(UpdateError::NotFoundUpdateTopicID(topic.id));
                    }
                    let id = topic.id.clone();
                    topics.insert(id, topic);
                    topics
                },
                ..(self.clone())
            },
        })
    }
}
