use crate::prelude::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let shared_state = use_context::<StateContext>().unwrap().state;
    let mut todos = shared_state
        .todos
        .iter()
        .map(|(id, todo)| (id.to_owned(), todo.clone()))
        .collect::<Vec<(TodoID, Todo)>>();
    let activities = shared_state
        .activities
        .iter()
        .map(|(id, activity)| (id.to_owned(), activity.clone()))
        .collect::<Vec<(ActivityID, Activity)>>();
    let topics = shared_state
        .topics
        .iter()
        .map(|(id, activity)| (id.to_owned(), activity.clone()))
        .collect::<Vec<(TopicID, Topic)>>();
    todos.sort_by(|(_, todo1), (_, todo2)| todo1.created_at.cmp(&todo2.created_at).reverse());

    html! {
        <div class="container">
            <div>
                <h2>{"Todos"}</h2>
                { for todos.iter().map(|(id, todo)| {
                    html! {
                        <div key={id.clone().0}>
                            <Link<AppRoute> to={AppRoute::TodoEdit { query: todo.id.abbreviate().to_owned() }}>
                                {&todo.id.abbreviate()}
                                <input type="checkbox" checked={todo.completed} disabled={true} />
                                <b>{&todo.title}</b>
                                {" "}
                                <span>{&todo.description}</span>
                            </Link<AppRoute>>
                        </div>
                    }
                }) }
                { if todos.len() == 0 { html! { <p>{ "No todos" }</p> } } else { "".into() } }

                <b>{ "Add Todo" }</b>
                <TodoAdder />
            </div>

            <hr />

            <div>
                <h2>{"Activities"}</h2>
                { for activities.iter().map(|(id, activity)| {
                    html! {
                        <div key={id.clone().0}>
                            <b>{&activity.title}</b>
                        </div>
                    }
                }) }
                { if activities.len() == 0 { html! { <p>{ "No activities" }</p> } } else { "".into() } }
            </div>

            <hr />

            <div>
                <h2>{"Topics"}</h2>
                { for topics.iter().map(|(id, topic)| {
                    html! {
                        <div key={id.clone().0}>
                            <b>{&topic.name}</b>
                        </div>
                    }
                }) }
                { if topics.len() == 0 { html! { <p>{ "No topics" }</p> } } else { "".into() } }
            </div>
        </div>
    }
}

#[function_component(TodoAdder)]
pub fn todo_adder() -> Html {
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
