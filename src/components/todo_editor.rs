use crate::prelude::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct TodoEditorQueryProps {
    pub query: String,
}

#[function_component(TodoEditorQuery)]
pub fn todo_editor_query(props: &TodoEditorQueryProps) -> Html {
    let shared_state = use_context::<StateContext>().unwrap().state;
    let matching_todos = shared_state
        .todos
        .iter()
        .filter(|(todo_id, _todo)| todo_id.is_abbreviated_by(&props.query))
        .collect::<Vec<_>>();

    if matching_todos.len() == 0 {
        html! {
            <div>{"Error: Todo not found"}</div>
        }
    } else {
        html! {
            <TodoEditor todo_id={matching_todos[0].0.clone()} />
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct TodoEditorProps {
    pub todo_id: TodoID,
}

#[function_component(TodoEditor)]
pub fn todo_editor(props: &TodoEditorProps) -> Html {
    html! {
        <div>{format!("Todo Editor for {}", props.todo_id)}</div>
    }
}
