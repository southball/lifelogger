use crate::components::*;
use crate::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/todo/:query/edit")]
    TodoEdit { query: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &AppRoute) -> Html {
    match route {
        AppRoute::Home => html! { <Home /> },
        AppRoute::TodoEdit { query } => html! { <TodoEditorQuery query={query.to_owned()} /> },
        AppRoute::NotFound => html! {"Not found!"},
    }
}
