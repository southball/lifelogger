use super::display::*;
use super::editor::*;
use crate::prelude::*;
use yew::prelude::*;

pub enum HomeMsg {
    Mutation(Mutation),
}

pub struct Home {}

impl Component for Home {
    type Message = HomeMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Display />
                <Editor />
            </>
        }
    }
}
