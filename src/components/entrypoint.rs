use super::home::*;
use crate::prelude::*;
use yew::prelude::*;

#[function_component(Entrypoint)]
pub fn entrypoint() -> Html {
    let state = use_state(|| SharedState::default());
    let mutate = {
        let state = state.clone();
        Callback::from(move |mutation| {
            let new_state = state.update(mutation).unwrap();
            web_sys::console::log_1(&format!("{:#?}", new_state).into());
            state.set(new_state)
        })
    };

    let context = StateContext {
        state: (*state).clone(),
        mutate,
    };

    html! {
        <ContextProvider<StateContext> {context}>
            <Home />
        </ContextProvider<StateContext>>
    }
}
