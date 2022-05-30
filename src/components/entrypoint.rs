use super::home::*;
use crate::context::*;
use crate::models::*;
use yew::prelude::*;

#[function_component(Entrypoint)]
pub fn entrypoint() -> Html {
    let state = use_state(|| SharedState::default());
    let mutate = {
        let state = state.clone();
        Callback::from(move |mutation| state.set(state.update(mutation).unwrap()))
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
