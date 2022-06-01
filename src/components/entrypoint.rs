use super::home::*;
use crate::prelude::*;
use yew::prelude::*;

const STORAGE_KEY: &'static str = "LIFELOGGER_DATA";

#[function_component(Entrypoint)]
pub fn entrypoint() -> Html {
    let state = use_state(|| {
        let storage = web_sys::window().unwrap().local_storage().unwrap();
        storage
            .and_then(|storage| storage.get(STORAGE_KEY).unwrap())
            .and_then(|json_data: String| {
                serde_json::from_str(&json_data)
                    .map_err(|error| {
                        web_sys::console::log_1(&format!("{:#?}", &error).into());
                        error
                    })
                    .ok()
            })
            .unwrap_or_else(|| SharedState::default())
    });

    let mutate = {
        let state = state.clone();
        Callback::from(move |mutation| {
            let new_state = state.update(mutation).unwrap();
            state.set(new_state.clone());

            let storage = web_sys::window().unwrap().local_storage().unwrap();
            storage.map(|storage| {
                storage
                    .set(STORAGE_KEY, &serde_json::to_string(&new_state).unwrap())
                    .unwrap();
            });
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
