use super::display::*;
use super::editor::*;
use crate::models::*;
use yew::prelude::*;

pub enum HomeMsg {
    Mutation(Mutation),
}

pub struct Home {
    pub state: SharedState,
}

impl Component for Home {
    type Message = HomeMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: Default::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HomeMsg::Mutation(mutation) => {
                self.state = self.state.update(mutation).unwrap();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_mutation = ctx.link().callback(|mutation| HomeMsg::Mutation(mutation));

        html! {
            <>
                <Display shared_state={self.state.clone()} />
                <Editor on_mutation={on_mutation} />
            </>
        }
    }
}
