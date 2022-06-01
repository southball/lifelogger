use super::display::*;
use super::editor::*;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container">
            <Display />
            <Editor />
        </div>
    }
}
