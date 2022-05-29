use crate::models::*;
use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct Editor {
    title_ref: NodeRef,
    description_ref: NodeRef,
}

pub enum EditorMsg {
    Add,
}

#[derive(Clone, PartialEq, Properties)]
pub struct EditorProps {
    pub on_mutation: Callback<Mutation>,
}

impl Component for Editor {
    type Message = EditorMsg;
    type Properties = EditorProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            title_ref: NodeRef::default(),
            description_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let title_input = self.title_ref.cast::<HtmlInputElement>().unwrap();
        let description_input = self.description_ref.cast::<HtmlInputElement>().unwrap();
        match msg {
            EditorMsg::Add => {
                let title = title_input.value();
                let description = description_input.value();
                web_sys::console::log_2(&title.clone().into(), &description.clone().into());
                title_input.set_value("");
                description_input.set_value("");
                ctx.props()
                    .on_mutation
                    .emit(Mutation::Todo(TodoMutation::Add(Todo {
                        id: Uuid::new_v4().to_string(),
                        title,
                        description,
                        completed: false,
                    })));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| EditorMsg::Add);
        html! {
            <div>
                <input type="text" placeholder="Title" ref={self.title_ref.clone()} />
                <input type="text" placeholder="Description" ref={self.description_ref.clone()} />
                <button type="button" onclick={onclick}>{"Add"}</button>
            </div>
        }
    }
}
