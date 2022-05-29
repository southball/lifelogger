use crate::models::*;
use yew::prelude::*;

pub struct Display {}

pub enum DisplayMsg {}

#[derive(Clone, PartialEq, Properties)]
pub struct DisplayProps {
    pub shared_state: SharedState,
}

impl Component for Display {
    type Message = DisplayMsg;
    type Properties = DisplayProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let shared_state = &ctx.props().shared_state;
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
