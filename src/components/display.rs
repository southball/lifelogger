use crate::prelude::*;
use yew::prelude::*;

#[function_component(Display)]
pub fn display() -> Html {
    let shared_state = use_context::<StateContext>().unwrap().state;
    let mut todos = shared_state
        .todos
        .iter()
        .map(|(id, todo)| (id.to_owned(), todo.clone()))
        .collect::<Vec<(String, Todo)>>();
    todos.sort_by(|(_, todo1), (_, todo2)| todo1.created_at.cmp(&todo2.created_at).reverse());

    html! {
        <div>
            <h2>{"Todos"}</h2>
            { for todos.iter().map(|(id, todo)| {
                html! { <TodoDisplay key={id.as_ref()} todo={todo.clone()} /> }
            }) }
            { if todos.len() == 0 { "No todos" } else { "" } }
            <hr />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct TodoDisplayProps {
    pub todo: Todo,
}

#[function_component(TodoDisplay)]
fn todo_display(props: &TodoDisplayProps) -> Html {
    let context = use_context::<StateContext>().unwrap();
    let todo = &props.todo;
    let on_checkbox_change = {
        let todo = todo.clone();
        Callback::from(move |_event: web_sys::Event| {
            context.mutate.emit(
                TodoMutation::Update(Todo {
                    completed: !todo.completed,
                    ..(todo.clone())
                })
                .into(),
            );
        })
    };
    html! {
        <div>
            <p>
                <input type="checkbox" checked={todo.completed} onchange={on_checkbox_change} />
                <b>{&todo.title}</b>
                {" "}
                <span>{&todo.description}</span>
            </p>
        </div>
    }
}
