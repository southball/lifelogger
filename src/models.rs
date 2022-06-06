use derive_more::Display;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RepeatingType {
    EveryNDays {
        count: u32,
    },
    EveryNWeeks {
        count: u32,
        /// 0 is Sunday, 1 is Monday, etc.
        days: [bool; 7],
    },
    EveryNMonthsDate {
        count: u32,
        date: u32,
    },
    EveryNMonthsWeekDay {
        count: u32,
        week: u32,
        day: u32,
    },
    EveryNYears {
        count: u32,
    },
}

#[derive(Clone, Debug, Display, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct TopicID(pub String);

#[derive(Clone, Debug, PartialEq, Builder, Serialize, Deserialize)]
pub struct Topic {
    #[builder(default = "TopicID(uuid::Uuid::new_v4().to_string())")]
    pub id: TopicID,
    pub name: String,
    #[builder(default)]
    pub parent_id: Option<String>,
    #[builder(default = "false")]
    pub archived: bool,
}

#[derive(Clone, Debug, Display, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct TodoID(pub String);

#[derive(Clone, Debug, PartialEq, Builder, Serialize, Deserialize)]
pub struct Todo {
    #[builder(default = "TodoID(uuid::Uuid::new_v4().to_string())")]
    pub id: TodoID,
    pub title: String,
    pub description: String,
    #[builder(default = "false")]
    pub completed: bool,
    #[builder(default = "chrono::Utc::now()")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[builder(default)]
    pub deadline: Option<chrono::DateTime<chrono::Utc>>,
    #[builder(default)]
    pub topic_id: Option<TopicID>,
    #[builder(default = "false")]
    pub archived: bool,
}

#[derive(Clone, Debug, Display, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct EventID(pub String);

#[derive(Clone, Debug, PartialEq, Builder, Serialize, Deserialize)]
pub struct Event {
    #[builder(default = "EventID(uuid::Uuid::new_v4().to_string())")]
    pub id: EventID,
    pub title: String,
    pub description: String,
    #[builder(default)]
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    #[builder(default)]
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    #[builder(default)]
    pub repeating_type: Option<RepeatingType>,
    #[builder(default)]
    pub topic_id: Option<TopicID>,
    #[builder(default = "false")]
    pub archived: bool,
}

#[derive(Clone, Debug, Display, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct ActivityID(pub String);

#[derive(Clone, Debug, PartialEq, Builder, Serialize, Deserialize)]
pub struct Activity {
    #[builder(default = "ActivityID(uuid::Uuid::new_v4().to_string())")]
    pub id: ActivityID,
    pub title: String,
    pub description: String,
    #[builder(default)]
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    #[builder(default)]
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    #[builder(default)]
    pub topic_id: Option<TopicID>,
    #[builder(default)]
    pub todo_id: Option<TodoID>,
    #[builder(default)]
    pub event_id: Option<EventID>,
    #[builder(default = "false")]
    pub archived: bool,
}

/// This is the shared state that is shared across client and server.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SharedState {
    pub topics: im::HashMap<TopicID, Topic>,
    pub todos: im::HashMap<TodoID, Todo>,
    pub events: im::HashMap<EventID, Event>,
    pub activities: im::HashMap<ActivityID, Activity>,
}

pub enum TopicMutation {
    Add(Topic),
    Update(Topic),
}

pub enum TodoMutation {
    Add(Todo),
    Update(Todo),
}

pub enum EventMutation {
    Add(Event),
    Update(Event),
}

pub enum ActivityMutation {
    Add(Activity),
    Update(Activity),
}

pub enum Mutation {
    Topic(TopicMutation),
    Todo(TodoMutation),
    Event(EventMutation),
    Activity(ActivityMutation),
}

impl From<TodoMutation> for Mutation {
    fn from(todo_mutation: TodoMutation) -> Mutation {
        Mutation::Todo(todo_mutation)
    }
}

impl From<TopicMutation> for Mutation {
    fn from(topic_mutation: TopicMutation) -> Mutation {
        Mutation::Topic(topic_mutation)
    }
}

impl From<EventMutation> for Mutation {
    fn from(event_mutation: EventMutation) -> Mutation {
        Mutation::Event(event_mutation)
    }
}

impl From<ActivityMutation> for Mutation {
    fn from(activity_mutation: ActivityMutation) -> Mutation {
        Mutation::Activity(activity_mutation)
    }
}

#[derive(Error, Debug)]
pub enum UpdateError {
    #[error("Duplicate ID {0} when adding topic")]
    DuplicateAddTopicID(TopicID),
    #[error("ID {0} not found when updating topic")]
    NotFoundUpdateTopicID(TopicID),
    #[error("Duplicate ID {0} when adding todo")]
    DuplicateAddTodoID(TodoID),
    #[error("ID {0} not found when updating todo")]
    NotFoundUpdateTodoID(TodoID),
    #[error("Duplicate ID {0} when adding event")]
    DuplicateAddEventID(EventID),
    #[error("ID {0} not found when updating event")]
    NotFoundUpdateEventID(EventID),
    #[error("Duplicate ID {0} when adding activity")]
    DuplicateAddActivityID(ActivityID),
    #[error("ID {0} not found when updating activity")]
    NotFoundUpdateActivityID(ActivityID),
}

impl SharedState {
    pub fn update(&self, mutation: Mutation) -> Result<SharedState, UpdateError> {
        Ok(match mutation {
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
            Mutation::Event(EventMutation::Add(event)) => SharedState {
                events: {
                    let mut events = self.events.clone();
                    if events.contains_key(&event.id) {
                        return Err(UpdateError::DuplicateAddEventID(event.id));
                    }
                    let id = event.id.clone();
                    events.insert(id, event);
                    events
                },
                ..(self.clone())
            },
            Mutation::Event(EventMutation::Update(event)) => SharedState {
                events: {
                    let mut events = self.events.clone();
                    if !events.contains_key(&event.id) {
                        return Err(UpdateError::NotFoundUpdateEventID(event.id));
                    }
                    let id = event.id.clone();
                    events.insert(id, event);
                    events
                },
                ..(self.clone())
            },
        })
    }
}
