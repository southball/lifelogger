use super::display::*;
use super::editor::*;
use crate::models::*;
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

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Display />
                <Editor />
            </>
        }
    }
}
