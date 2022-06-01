use crate::prelude::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(Editor)]
pub fn editor() -> Html {
    let title_ref = use_node_ref();
    let description_ref = use_node_ref();

    let state_context = use_context::<StateContext>().unwrap();

    let onsubmit = {
        let title_ref = title_ref.clone();
        let description_ref = description_ref.clone();
        Callback::from(move |event: web_sys::FocusEvent| {
            event.prevent_default();

            let title_input = title_ref.cast::<HtmlInputElement>().unwrap();
            let description_input = description_ref.cast::<HtmlInputElement>().unwrap();

            let title = title_input.value();
            let description = description_input.value();
            title_input.set_value("");
            description_input.set_value("");
            state_context.mutate.emit(Mutation::Todo(TodoMutation::Add(
                TodoBuilder::default()
                    .title(title)
                    .description(description)
                    .build()
                    .unwrap(),
            )));
            title_input.focus().unwrap();
        })
    };

    html! {
        <div>
            <form onsubmit={onsubmit}>
                <input type="text" placeholder="Title" ref={title_ref} />
                <input type="text" placeholder="Description" ref={description_ref} />
                <button type="submit">{"Add"}</button>
            </form>
        </div>
    }
}
