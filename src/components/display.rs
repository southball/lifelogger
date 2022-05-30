use crate::context::*;
use crate::models::*;
use yew::prelude::*;

#[function_component(Display)]
pub fn display() -> Html {
    let shared_state = use_context::<StateContext>().unwrap().state;

    html! {
        <div>
            <h2>{"Todos"}</h2>
            { for shared_state.todos.iter().map(|(id, todo)| {
                html! { <TodoDisplay key={id.as_ref()} todo={todo.clone()} /> }
            }) }
            { if shared_state.todos.len() == 0 { "No todos" } else { "" } }
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
    let todo = &props.todo;
    html! {
        <div>
            <p>
                <input type="checkbox" checked={todo.completed} />
                <b>{&todo.title}</b>
                {" "}
                <span>{&todo.description}</span>
            </p>
        </div>
    }
}
